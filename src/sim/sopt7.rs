#[doc = "Reader of register SOPT7"]
pub type R = crate::R<u32, super::SOPT7>;
#[doc = "Writer for register SOPT7"]
pub type W = crate::W<u32, super::SOPT7>;
#[doc = "Register SOPT7 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC0 trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0TRGSEL_A {
    #[doc = "0: PDB external trigger pin input (PDB0_EXTRG)"]
    _0000,
    #[doc = "1: High speed comparator 0 output"]
    _0001,
    #[doc = "2: High speed comparator 1 output"]
    _0010,
    #[doc = "3: High speed comparator 2 output"]
    _0011,
    #[doc = "4: PIT trigger 0"]
    _0100,
    #[doc = "5: PIT trigger 1"]
    _0101,
    #[doc = "6: PIT trigger 2"]
    _0110,
    #[doc = "7: PIT trigger 3"]
    _0111,
    #[doc = "8: FTM0 trigger"]
    _1000,
    #[doc = "9: FTM1 trigger"]
    _1001,
    #[doc = "10: FTM2 trigger"]
    _1010,
    #[doc = "11: FTM3 trigger"]
    _1011,
    #[doc = "12: RTC alarm"]
    _1100,
    #[doc = "13: RTC seconds"]
    _1101,
    #[doc = "14: Low-power timer (LPTMR) trigger"]
    _1110,
    #[doc = "15: TPM1 channel 0 (A pretrigger) and channel 1 (B pretrigger)"]
    _1111,
}
impl From<ADC0TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0TRGSEL_A) -> Self {
        match variant {
            ADC0TRGSEL_A::_0000 => 0,
            ADC0TRGSEL_A::_0001 => 1,
            ADC0TRGSEL_A::_0010 => 2,
            ADC0TRGSEL_A::_0011 => 3,
            ADC0TRGSEL_A::_0100 => 4,
            ADC0TRGSEL_A::_0101 => 5,
            ADC0TRGSEL_A::_0110 => 6,
            ADC0TRGSEL_A::_0111 => 7,
            ADC0TRGSEL_A::_1000 => 8,
            ADC0TRGSEL_A::_1001 => 9,
            ADC0TRGSEL_A::_1010 => 10,
            ADC0TRGSEL_A::_1011 => 11,
            ADC0TRGSEL_A::_1100 => 12,
            ADC0TRGSEL_A::_1101 => 13,
            ADC0TRGSEL_A::_1110 => 14,
            ADC0TRGSEL_A::_1111 => 15,
        }
    }
}
#[doc = "Reader of field `ADC0TRGSEL`"]
pub type ADC0TRGSEL_R = crate::R<u8, ADC0TRGSEL_A>;
impl ADC0TRGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0TRGSEL_A {
        match self.bits {
            0 => ADC0TRGSEL_A::_0000,
            1 => ADC0TRGSEL_A::_0001,
            2 => ADC0TRGSEL_A::_0010,
            3 => ADC0TRGSEL_A::_0011,
            4 => ADC0TRGSEL_A::_0100,
            5 => ADC0TRGSEL_A::_0101,
            6 => ADC0TRGSEL_A::_0110,
            7 => ADC0TRGSEL_A::_0111,
            8 => ADC0TRGSEL_A::_1000,
            9 => ADC0TRGSEL_A::_1001,
            10 => ADC0TRGSEL_A::_1010,
            11 => ADC0TRGSEL_A::_1011,
            12 => ADC0TRGSEL_A::_1100,
            13 => ADC0TRGSEL_A::_1101,
            14 => ADC0TRGSEL_A::_1110,
            15 => ADC0TRGSEL_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == ADC0TRGSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == ADC0TRGSEL_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == ADC0TRGSEL_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == ADC0TRGSEL_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == ADC0TRGSEL_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == ADC0TRGSEL_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == ADC0TRGSEL_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == ADC0TRGSEL_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == ADC0TRGSEL_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == ADC0TRGSEL_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == ADC0TRGSEL_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == ADC0TRGSEL_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == ADC0TRGSEL_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == ADC0TRGSEL_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == ADC0TRGSEL_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == ADC0TRGSEL_A::_1111
    }
}
#[doc = "Write proxy for field `ADC0TRGSEL`"]
pub struct ADC0TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0TRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0TRGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PDB external trigger pin input (PDB0_EXTRG)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0000)
    }
    #[doc = "High speed comparator 0 output"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0001)
    }
    #[doc = "High speed comparator 1 output"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0010)
    }
    #[doc = "High speed comparator 2 output"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0011)
    }
    #[doc = "PIT trigger 0"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0100)
    }
    #[doc = "PIT trigger 1"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0101)
    }
    #[doc = "PIT trigger 2"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0110)
    }
    #[doc = "PIT trigger 3"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0111)
    }
    #[doc = "FTM0 trigger"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1000)
    }
    #[doc = "FTM1 trigger"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1001)
    }
    #[doc = "FTM2 trigger"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1010)
    }
    #[doc = "FTM3 trigger"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1011)
    }
    #[doc = "RTC alarm"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1100)
    }
    #[doc = "RTC seconds"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1101)
    }
    #[doc = "Low-power timer (LPTMR) trigger"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1110)
    }
    #[doc = "TPM1 channel 0 (A pretrigger) and channel 1 (B pretrigger)"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "ADC0 pretrigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0PRETRGSEL_A {
    #[doc = "0: Pre-trigger A"]
    _0,
    #[doc = "1: Pre-trigger B"]
    _1,
}
impl From<ADC0PRETRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0PRETRGSEL_A) -> Self {
        match variant {
            ADC0PRETRGSEL_A::_0 => false,
            ADC0PRETRGSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADC0PRETRGSEL`"]
pub type ADC0PRETRGSEL_R = crate::R<bool, ADC0PRETRGSEL_A>;
impl ADC0PRETRGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0PRETRGSEL_A {
        match self.bits {
            false => ADC0PRETRGSEL_A::_0,
            true => ADC0PRETRGSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC0PRETRGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC0PRETRGSEL_A::_1
    }
}
#[doc = "Write proxy for field `ADC0PRETRGSEL`"]
pub struct ADC0PRETRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0PRETRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0PRETRGSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pre-trigger A"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0PRETRGSEL_A::_0)
    }
    #[doc = "Pre-trigger B"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0PRETRGSEL_A::_1)
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
#[doc = "ADC0 alternate trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0ALTTRGEN_A {
    #[doc = "0: PDB trigger selected for ADC0."]
    _0,
    #[doc = "1: Alternate trigger selected for ADC0."]
    _1,
}
impl From<ADC0ALTTRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0ALTTRGEN_A) -> Self {
        match variant {
            ADC0ALTTRGEN_A::_0 => false,
            ADC0ALTTRGEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADC0ALTTRGEN`"]
pub type ADC0ALTTRGEN_R = crate::R<bool, ADC0ALTTRGEN_A>;
impl ADC0ALTTRGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0ALTTRGEN_A {
        match self.bits {
            false => ADC0ALTTRGEN_A::_0,
            true => ADC0ALTTRGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC0ALTTRGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC0ALTTRGEN_A::_1
    }
}
#[doc = "Write proxy for field `ADC0ALTTRGEN`"]
pub struct ADC0ALTTRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0ALTTRGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC0ALTTRGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB trigger selected for ADC0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0ALTTRGEN_A::_0)
    }
    #[doc = "Alternate trigger selected for ADC0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0ALTTRGEN_A::_1)
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
#[doc = "ADC1 trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1TRGSEL_A {
    #[doc = "0: PDB external trigger pin input (PDB0_EXTRG)"]
    _0000,
    #[doc = "1: High speed comparator 0 output"]
    _0001,
    #[doc = "2: High speed comparator 1 output"]
    _0010,
    #[doc = "3: High speed comparator 2 output"]
    _0011,
    #[doc = "4: PIT trigger 0"]
    _0100,
    #[doc = "5: PIT trigger 1"]
    _0101,
    #[doc = "6: PIT trigger 2"]
    _0110,
    #[doc = "7: PIT trigger 3"]
    _0111,
    #[doc = "8: FTM0 trigger"]
    _1000,
    #[doc = "9: FTM1 trigger"]
    _1001,
    #[doc = "10: FTM2 trigger"]
    _1010,
    #[doc = "11: FTM3 trigger"]
    _1011,
    #[doc = "12: RTC alarm"]
    _1100,
    #[doc = "13: RTC seconds"]
    _1101,
    #[doc = "14: Low-power timer (LPTMR) trigger"]
    _1110,
    #[doc = "15: TPM2 channel 0 (A pretrigger) and channel 1 (B pretrigger)"]
    _1111,
}
impl From<ADC1TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC1TRGSEL_A) -> Self {
        match variant {
            ADC1TRGSEL_A::_0000 => 0,
            ADC1TRGSEL_A::_0001 => 1,
            ADC1TRGSEL_A::_0010 => 2,
            ADC1TRGSEL_A::_0011 => 3,
            ADC1TRGSEL_A::_0100 => 4,
            ADC1TRGSEL_A::_0101 => 5,
            ADC1TRGSEL_A::_0110 => 6,
            ADC1TRGSEL_A::_0111 => 7,
            ADC1TRGSEL_A::_1000 => 8,
            ADC1TRGSEL_A::_1001 => 9,
            ADC1TRGSEL_A::_1010 => 10,
            ADC1TRGSEL_A::_1011 => 11,
            ADC1TRGSEL_A::_1100 => 12,
            ADC1TRGSEL_A::_1101 => 13,
            ADC1TRGSEL_A::_1110 => 14,
            ADC1TRGSEL_A::_1111 => 15,
        }
    }
}
#[doc = "Reader of field `ADC1TRGSEL`"]
pub type ADC1TRGSEL_R = crate::R<u8, ADC1TRGSEL_A>;
impl ADC1TRGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1TRGSEL_A {
        match self.bits {
            0 => ADC1TRGSEL_A::_0000,
            1 => ADC1TRGSEL_A::_0001,
            2 => ADC1TRGSEL_A::_0010,
            3 => ADC1TRGSEL_A::_0011,
            4 => ADC1TRGSEL_A::_0100,
            5 => ADC1TRGSEL_A::_0101,
            6 => ADC1TRGSEL_A::_0110,
            7 => ADC1TRGSEL_A::_0111,
            8 => ADC1TRGSEL_A::_1000,
            9 => ADC1TRGSEL_A::_1001,
            10 => ADC1TRGSEL_A::_1010,
            11 => ADC1TRGSEL_A::_1011,
            12 => ADC1TRGSEL_A::_1100,
            13 => ADC1TRGSEL_A::_1101,
            14 => ADC1TRGSEL_A::_1110,
            15 => ADC1TRGSEL_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == ADC1TRGSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == ADC1TRGSEL_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == ADC1TRGSEL_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == ADC1TRGSEL_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == ADC1TRGSEL_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == ADC1TRGSEL_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == ADC1TRGSEL_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == ADC1TRGSEL_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == ADC1TRGSEL_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == ADC1TRGSEL_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == ADC1TRGSEL_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == ADC1TRGSEL_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == ADC1TRGSEL_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == ADC1TRGSEL_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == ADC1TRGSEL_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == ADC1TRGSEL_A::_1111
    }
}
#[doc = "Write proxy for field `ADC1TRGSEL`"]
pub struct ADC1TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1TRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC1TRGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "PDB external trigger pin input (PDB0_EXTRG)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0000)
    }
    #[doc = "High speed comparator 0 output"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0001)
    }
    #[doc = "High speed comparator 1 output"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0010)
    }
    #[doc = "High speed comparator 2 output"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0011)
    }
    #[doc = "PIT trigger 0"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0100)
    }
    #[doc = "PIT trigger 1"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0101)
    }
    #[doc = "PIT trigger 2"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0110)
    }
    #[doc = "PIT trigger 3"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0111)
    }
    #[doc = "FTM0 trigger"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1000)
    }
    #[doc = "FTM1 trigger"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1001)
    }
    #[doc = "FTM2 trigger"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1010)
    }
    #[doc = "FTM3 trigger"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1011)
    }
    #[doc = "RTC alarm"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1100)
    }
    #[doc = "RTC seconds"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1101)
    }
    #[doc = "Low-power timer (LPTMR) trigger"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1110)
    }
    #[doc = "TPM2 channel 0 (A pretrigger) and channel 1 (B pretrigger)"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "ADC1 pre-trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1PRETRGSEL_A {
    #[doc = "0: Pre-trigger A selected for ADC1."]
    _0,
    #[doc = "1: Pre-trigger B selected for ADC1."]
    _1,
}
impl From<ADC1PRETRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1PRETRGSEL_A) -> Self {
        match variant {
            ADC1PRETRGSEL_A::_0 => false,
            ADC1PRETRGSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADC1PRETRGSEL`"]
pub type ADC1PRETRGSEL_R = crate::R<bool, ADC1PRETRGSEL_A>;
impl ADC1PRETRGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1PRETRGSEL_A {
        match self.bits {
            false => ADC1PRETRGSEL_A::_0,
            true => ADC1PRETRGSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC1PRETRGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC1PRETRGSEL_A::_1
    }
}
#[doc = "Write proxy for field `ADC1PRETRGSEL`"]
pub struct ADC1PRETRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1PRETRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC1PRETRGSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pre-trigger A selected for ADC1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC1PRETRGSEL_A::_0)
    }
    #[doc = "Pre-trigger B selected for ADC1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC1PRETRGSEL_A::_1)
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
#[doc = "ADC1 alternate trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1ALTTRGEN_A {
    #[doc = "0: PDB trigger selected for ADC1"]
    _0,
    #[doc = "1: Alternate trigger selected for ADC1 as defined by ADC1TRGSEL."]
    _1,
}
impl From<ADC1ALTTRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1ALTTRGEN_A) -> Self {
        match variant {
            ADC1ALTTRGEN_A::_0 => false,
            ADC1ALTTRGEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADC1ALTTRGEN`"]
pub type ADC1ALTTRGEN_R = crate::R<bool, ADC1ALTTRGEN_A>;
impl ADC1ALTTRGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1ALTTRGEN_A {
        match self.bits {
            false => ADC1ALTTRGEN_A::_0,
            true => ADC1ALTTRGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC1ALTTRGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC1ALTTRGEN_A::_1
    }
}
#[doc = "Write proxy for field `ADC1ALTTRGEN`"]
pub struct ADC1ALTTRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1ALTTRGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC1ALTTRGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB trigger selected for ADC1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC1ALTTRGEN_A::_0)
    }
    #[doc = "Alternate trigger selected for ADC1 as defined by ADC1TRGSEL."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC1ALTTRGEN_A::_1)
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
impl R {
    #[doc = "Bits 0:3 - ADC0 trigger select"]
    #[inline(always)]
    pub fn adc0trgsel(&self) -> ADC0TRGSEL_R {
        ADC0TRGSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - ADC0 pretrigger select"]
    #[inline(always)]
    pub fn adc0pretrgsel(&self) -> ADC0PRETRGSEL_R {
        ADC0PRETRGSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC0 alternate trigger enable"]
    #[inline(always)]
    pub fn adc0alttrgen(&self) -> ADC0ALTTRGEN_R {
        ADC0ALTTRGEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - ADC1 trigger select"]
    #[inline(always)]
    pub fn adc1trgsel(&self) -> ADC1TRGSEL_R {
        ADC1TRGSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - ADC1 pre-trigger select"]
    #[inline(always)]
    pub fn adc1pretrgsel(&self) -> ADC1PRETRGSEL_R {
        ADC1PRETRGSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 15 - ADC1 alternate trigger enable"]
    #[inline(always)]
    pub fn adc1alttrgen(&self) -> ADC1ALTTRGEN_R {
        ADC1ALTTRGEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC0 trigger select"]
    #[inline(always)]
    pub fn adc0trgsel(&mut self) -> ADC0TRGSEL_W {
        ADC0TRGSEL_W { w: self }
    }
    #[doc = "Bit 4 - ADC0 pretrigger select"]
    #[inline(always)]
    pub fn adc0pretrgsel(&mut self) -> ADC0PRETRGSEL_W {
        ADC0PRETRGSEL_W { w: self }
    }
    #[doc = "Bit 7 - ADC0 alternate trigger enable"]
    #[inline(always)]
    pub fn adc0alttrgen(&mut self) -> ADC0ALTTRGEN_W {
        ADC0ALTTRGEN_W { w: self }
    }
    #[doc = "Bits 8:11 - ADC1 trigger select"]
    #[inline(always)]
    pub fn adc1trgsel(&mut self) -> ADC1TRGSEL_W {
        ADC1TRGSEL_W { w: self }
    }
    #[doc = "Bit 12 - ADC1 pre-trigger select"]
    #[inline(always)]
    pub fn adc1pretrgsel(&mut self) -> ADC1PRETRGSEL_W {
        ADC1PRETRGSEL_W { w: self }
    }
    #[doc = "Bit 15 - ADC1 alternate trigger enable"]
    #[inline(always)]
    pub fn adc1alttrgen(&mut self) -> ADC1ALTTRGEN_W {
        ADC1ALTTRGEN_W { w: self }
    }
}
