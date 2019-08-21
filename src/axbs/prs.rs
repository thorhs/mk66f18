#[doc = "Reader of register PRS%s"]
pub type R = crate::R<u32, super::PRS>;
#[doc = "Writer for register PRS%s"]
pub type W = crate::W<u32, super::PRS>;
#[doc = "Register PRS%s `reset()`'s with value 0x0654_3210"]
impl crate::ResetValue for super::PRS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0654_3210
    }
}
#[doc = "Master 0 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M0_A {
    #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "1: This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "2: This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "3: This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "4: This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "5: This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "6: This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl From<M0_A> for u8 {
    #[inline(always)]
    fn from(variant: M0_A) -> Self {
        match variant {
            M0_A::_000 => 0,
            M0_A::_001 => 1,
            M0_A::_010 => 2,
            M0_A::_011 => 3,
            M0_A::_100 => 4,
            M0_A::_101 => 5,
            M0_A::_110 => 6,
            M0_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `M0`"]
pub type M0_R = crate::R<u8, M0_A>;
impl M0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M0_A {
        match self.bits {
            0 => M0_A::_000,
            1 => M0_A::_001,
            2 => M0_A::_010,
            3 => M0_A::_011,
            4 => M0_A::_100,
            5 => M0_A::_101,
            6 => M0_A::_110,
            7 => M0_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == M0_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == M0_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == M0_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == M0_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == M0_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == M0_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == M0_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == M0_A::_111
    }
}
#[doc = "Write proxy for field `M0`"]
pub struct M0_W<'a> {
    w: &'a mut W,
}
impl<'a> M0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(M0_A::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(M0_A::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(M0_A::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(M0_A::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(M0_A::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(M0_A::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(M0_A::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(M0_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Master 1 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M1_A {
    #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "1: This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "2: This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "3: This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "4: This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "5: This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "6: This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl From<M1_A> for u8 {
    #[inline(always)]
    fn from(variant: M1_A) -> Self {
        match variant {
            M1_A::_000 => 0,
            M1_A::_001 => 1,
            M1_A::_010 => 2,
            M1_A::_011 => 3,
            M1_A::_100 => 4,
            M1_A::_101 => 5,
            M1_A::_110 => 6,
            M1_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `M1`"]
pub type M1_R = crate::R<u8, M1_A>;
impl M1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M1_A {
        match self.bits {
            0 => M1_A::_000,
            1 => M1_A::_001,
            2 => M1_A::_010,
            3 => M1_A::_011,
            4 => M1_A::_100,
            5 => M1_A::_101,
            6 => M1_A::_110,
            7 => M1_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == M1_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == M1_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == M1_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == M1_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == M1_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == M1_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == M1_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == M1_A::_111
    }
}
#[doc = "Write proxy for field `M1`"]
pub struct M1_W<'a> {
    w: &'a mut W,
}
impl<'a> M1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(M1_A::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(M1_A::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(M1_A::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(M1_A::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(M1_A::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(M1_A::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(M1_A::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(M1_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Master 2 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M2_A {
    #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "1: This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "2: This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "3: This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "4: This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "5: This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "6: This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl From<M2_A> for u8 {
    #[inline(always)]
    fn from(variant: M2_A) -> Self {
        match variant {
            M2_A::_000 => 0,
            M2_A::_001 => 1,
            M2_A::_010 => 2,
            M2_A::_011 => 3,
            M2_A::_100 => 4,
            M2_A::_101 => 5,
            M2_A::_110 => 6,
            M2_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `M2`"]
pub type M2_R = crate::R<u8, M2_A>;
impl M2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M2_A {
        match self.bits {
            0 => M2_A::_000,
            1 => M2_A::_001,
            2 => M2_A::_010,
            3 => M2_A::_011,
            4 => M2_A::_100,
            5 => M2_A::_101,
            6 => M2_A::_110,
            7 => M2_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == M2_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == M2_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == M2_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == M2_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == M2_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == M2_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == M2_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == M2_A::_111
    }
}
#[doc = "Write proxy for field `M2`"]
pub struct M2_W<'a> {
    w: &'a mut W,
}
impl<'a> M2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(M2_A::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(M2_A::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(M2_A::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(M2_A::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(M2_A::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(M2_A::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(M2_A::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(M2_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Master 3 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M3_A {
    #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "1: This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "2: This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "3: This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "4: This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "5: This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "6: This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl From<M3_A> for u8 {
    #[inline(always)]
    fn from(variant: M3_A) -> Self {
        match variant {
            M3_A::_000 => 0,
            M3_A::_001 => 1,
            M3_A::_010 => 2,
            M3_A::_011 => 3,
            M3_A::_100 => 4,
            M3_A::_101 => 5,
            M3_A::_110 => 6,
            M3_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `M3`"]
pub type M3_R = crate::R<u8, M3_A>;
impl M3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M3_A {
        match self.bits {
            0 => M3_A::_000,
            1 => M3_A::_001,
            2 => M3_A::_010,
            3 => M3_A::_011,
            4 => M3_A::_100,
            5 => M3_A::_101,
            6 => M3_A::_110,
            7 => M3_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == M3_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == M3_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == M3_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == M3_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == M3_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == M3_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == M3_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == M3_A::_111
    }
}
#[doc = "Write proxy for field `M3`"]
pub struct M3_W<'a> {
    w: &'a mut W,
}
impl<'a> M3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(M3_A::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(M3_A::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(M3_A::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(M3_A::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(M3_A::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(M3_A::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(M3_A::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(M3_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Master 4 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M4_A {
    #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "1: This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "2: This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "3: This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "4: This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "5: This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "6: This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl From<M4_A> for u8 {
    #[inline(always)]
    fn from(variant: M4_A) -> Self {
        match variant {
            M4_A::_000 => 0,
            M4_A::_001 => 1,
            M4_A::_010 => 2,
            M4_A::_011 => 3,
            M4_A::_100 => 4,
            M4_A::_101 => 5,
            M4_A::_110 => 6,
            M4_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `M4`"]
pub type M4_R = crate::R<u8, M4_A>;
impl M4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4_A {
        match self.bits {
            0 => M4_A::_000,
            1 => M4_A::_001,
            2 => M4_A::_010,
            3 => M4_A::_011,
            4 => M4_A::_100,
            5 => M4_A::_101,
            6 => M4_A::_110,
            7 => M4_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == M4_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == M4_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == M4_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == M4_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == M4_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == M4_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == M4_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == M4_A::_111
    }
}
#[doc = "Write proxy for field `M4`"]
pub struct M4_W<'a> {
    w: &'a mut W,
}
impl<'a> M4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(M4_A::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(M4_A::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(M4_A::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(M4_A::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(M4_A::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(M4_A::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(M4_A::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(M4_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Master 5 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M5_A {
    #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "1: This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "2: This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "3: This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "4: This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "5: This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "6: This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl From<M5_A> for u8 {
    #[inline(always)]
    fn from(variant: M5_A) -> Self {
        match variant {
            M5_A::_000 => 0,
            M5_A::_001 => 1,
            M5_A::_010 => 2,
            M5_A::_011 => 3,
            M5_A::_100 => 4,
            M5_A::_101 => 5,
            M5_A::_110 => 6,
            M5_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `M5`"]
pub type M5_R = crate::R<u8, M5_A>;
impl M5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M5_A {
        match self.bits {
            0 => M5_A::_000,
            1 => M5_A::_001,
            2 => M5_A::_010,
            3 => M5_A::_011,
            4 => M5_A::_100,
            5 => M5_A::_101,
            6 => M5_A::_110,
            7 => M5_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == M5_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == M5_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == M5_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == M5_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == M5_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == M5_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == M5_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == M5_A::_111
    }
}
#[doc = "Write proxy for field `M5`"]
pub struct M5_W<'a> {
    w: &'a mut W,
}
impl<'a> M5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(M5_A::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(M5_A::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(M5_A::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(M5_A::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(M5_A::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(M5_A::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(M5_A::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(M5_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Master 6 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M6_A {
    #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
    _000,
    #[doc = "1: This master has level 2 priority when accessing the slave port."]
    _001,
    #[doc = "2: This master has level 3 priority when accessing the slave port."]
    _010,
    #[doc = "3: This master has level 4 priority when accessing the slave port."]
    _011,
    #[doc = "4: This master has level 5 priority when accessing the slave port."]
    _100,
    #[doc = "5: This master has level 6 priority when accessing the slave port."]
    _101,
    #[doc = "6: This master has level 7 priority when accessing the slave port."]
    _110,
    #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
    _111,
}
impl From<M6_A> for u8 {
    #[inline(always)]
    fn from(variant: M6_A) -> Self {
        match variant {
            M6_A::_000 => 0,
            M6_A::_001 => 1,
            M6_A::_010 => 2,
            M6_A::_011 => 3,
            M6_A::_100 => 4,
            M6_A::_101 => 5,
            M6_A::_110 => 6,
            M6_A::_111 => 7,
        }
    }
}
#[doc = "Reader of field `M6`"]
pub type M6_R = crate::R<u8, M6_A>;
impl M6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M6_A {
        match self.bits {
            0 => M6_A::_000,
            1 => M6_A::_001,
            2 => M6_A::_010,
            3 => M6_A::_011,
            4 => M6_A::_100,
            5 => M6_A::_101,
            6 => M6_A::_110,
            7 => M6_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == M6_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == M6_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == M6_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == M6_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == M6_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == M6_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == M6_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == M6_A::_111
    }
}
#[doc = "Write proxy for field `M6`"]
pub struct M6_W<'a> {
    w: &'a mut W,
}
impl<'a> M6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(M6_A::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(M6_A::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(M6_A::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(M6_A::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(M6_A::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(M6_A::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(M6_A::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(M6_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Master 0 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Master 1 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Master 2 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m2(&self) -> M2_R {
        M2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Master 3 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m3(&self) -> M3_R {
        M3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Master 4 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m4(&self) -> M4_R {
        M4_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Master 5 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m5(&self) -> M5_R {
        M5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Master 6 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m6(&self) -> M6_R {
        M6_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Master 0 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m0(&mut self) -> M0_W {
        M0_W { w: self }
    }
    #[doc = "Bits 4:6 - Master 1 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m1(&mut self) -> M1_W {
        M1_W { w: self }
    }
    #[doc = "Bits 8:10 - Master 2 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m2(&mut self) -> M2_W {
        M2_W { w: self }
    }
    #[doc = "Bits 12:14 - Master 3 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m3(&mut self) -> M3_W {
        M3_W { w: self }
    }
    #[doc = "Bits 16:18 - Master 4 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m4(&mut self) -> M4_W {
        M4_W { w: self }
    }
    #[doc = "Bits 20:22 - Master 5 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m5(&mut self) -> M5_W {
        M5_W { w: self }
    }
    #[doc = "Bits 24:26 - Master 6 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m6(&mut self) -> M6_W {
        M6_W { w: self }
    }
}
