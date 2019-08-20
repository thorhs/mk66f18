#[doc = "Reader of register DATA"]
pub type R = crate::R<u32, super::DATA>;
#[doc = "Writer for register DATA"]
pub type W = crate::W<u32, super::DATA>;
#[doc = "Register DATA `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSICNT`"]
pub type TSICNT_R = crate::R<u16, u16>;
#[doc = "Software Trigger Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTS_AW {
    #[doc = "0: No effect."]
    _0,
    #[doc = "1: Start a scan to determine which channel is specified by TSI_DATA\\[TSICH\\]."]
    _1,
}
impl From<SWTS_AW> for bool {
    #[inline(always)]
    fn from(variant: SWTS_AW) -> Self {
        match variant {
            SWTS_AW::_0 => false,
            SWTS_AW::_1 => true,
        }
    }
}
#[doc = "Write proxy for field `SWTS`"]
pub struct SWTS_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWTS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWTS_AW::_0)
    }
    #[doc = "Start a scan to determine which channel is specified by TSI_DATA\\[TSICH\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWTS_AW::_1)
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
#[doc = "DMA Transfer Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: Interrupt is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
    _0,
    #[doc = "1: DMA transfer request is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
    _1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        match variant {
            DMAEN_A::_0 => false,
            DMAEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEN_A::_1
    }
}
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA transfer request is selected when the interrupt enable bit is set and the corresponding TSI events assert."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
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
#[doc = "TSICH\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSICH_A {
    #[doc = "0: Channel 0."]
    _0000,
    #[doc = "1: Channel 1."]
    _0001,
    #[doc = "2: Channel 2."]
    _0010,
    #[doc = "3: Channel 3."]
    _0011,
    #[doc = "4: Channel 4."]
    _0100,
    #[doc = "5: Channel 5."]
    _0101,
    #[doc = "6: Channel 6."]
    _0110,
    #[doc = "7: Channel 7."]
    _0111,
    #[doc = "8: Channel 8."]
    _1000,
    #[doc = "9: Channel 9."]
    _1001,
    #[doc = "10: Channel 10."]
    _1010,
    #[doc = "11: Channel 11."]
    _1011,
    #[doc = "12: Channel 12."]
    _1100,
    #[doc = "13: Channel 13."]
    _1101,
    #[doc = "14: Channel 14."]
    _1110,
    #[doc = "15: Channel 15."]
    _1111,
}
impl From<TSICH_A> for u8 {
    #[inline(always)]
    fn from(variant: TSICH_A) -> Self {
        match variant {
            TSICH_A::_0000 => 0,
            TSICH_A::_0001 => 1,
            TSICH_A::_0010 => 2,
            TSICH_A::_0011 => 3,
            TSICH_A::_0100 => 4,
            TSICH_A::_0101 => 5,
            TSICH_A::_0110 => 6,
            TSICH_A::_0111 => 7,
            TSICH_A::_1000 => 8,
            TSICH_A::_1001 => 9,
            TSICH_A::_1010 => 10,
            TSICH_A::_1011 => 11,
            TSICH_A::_1100 => 12,
            TSICH_A::_1101 => 13,
            TSICH_A::_1110 => 14,
            TSICH_A::_1111 => 15,
        }
    }
}
#[doc = "Reader of field `TSICH`"]
pub type TSICH_R = crate::R<u8, TSICH_A>;
impl TSICH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSICH_A {
        match self.bits {
            0 => TSICH_A::_0000,
            1 => TSICH_A::_0001,
            2 => TSICH_A::_0010,
            3 => TSICH_A::_0011,
            4 => TSICH_A::_0100,
            5 => TSICH_A::_0101,
            6 => TSICH_A::_0110,
            7 => TSICH_A::_0111,
            8 => TSICH_A::_1000,
            9 => TSICH_A::_1001,
            10 => TSICH_A::_1010,
            11 => TSICH_A::_1011,
            12 => TSICH_A::_1100,
            13 => TSICH_A::_1101,
            14 => TSICH_A::_1110,
            15 => TSICH_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TSICH_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == TSICH_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == TSICH_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == TSICH_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == TSICH_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == TSICH_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == TSICH_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == TSICH_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == TSICH_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == TSICH_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == TSICH_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == TSICH_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == TSICH_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == TSICH_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == TSICH_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == TSICH_A::_1111
    }
}
#[doc = "Write proxy for field `TSICH`"]
pub struct TSICH_W<'a> {
    w: &'a mut W,
}
impl<'a> TSICH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSICH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Channel 0."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TSICH_A::_0000)
    }
    #[doc = "Channel 1."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TSICH_A::_0001)
    }
    #[doc = "Channel 2."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TSICH_A::_0010)
    }
    #[doc = "Channel 3."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TSICH_A::_0011)
    }
    #[doc = "Channel 4."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(TSICH_A::_0100)
    }
    #[doc = "Channel 5."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(TSICH_A::_0101)
    }
    #[doc = "Channel 6."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(TSICH_A::_0110)
    }
    #[doc = "Channel 7."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(TSICH_A::_0111)
    }
    #[doc = "Channel 8."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(TSICH_A::_1000)
    }
    #[doc = "Channel 9."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(TSICH_A::_1001)
    }
    #[doc = "Channel 10."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(TSICH_A::_1010)
    }
    #[doc = "Channel 11."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(TSICH_A::_1011)
    }
    #[doc = "Channel 12."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(TSICH_A::_1100)
    }
    #[doc = "Channel 13."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(TSICH_A::_1101)
    }
    #[doc = "Channel 14."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(TSICH_A::_1110)
    }
    #[doc = "Channel 15."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(TSICH_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TSI Conversion Counter Value"]
    #[inline(always)]
    pub fn tsicnt(&self) -> TSICNT_R {
        TSICNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 23 - DMA Transfer Enabled"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - TSICH"]
    #[inline(always)]
    pub fn tsich(&self) -> TSICH_R {
        TSICH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 22 - Software Trigger Start"]
    #[inline(always)]
    pub fn swts(&mut self) -> SWTS_W {
        SWTS_W { w: self }
    }
    #[doc = "Bit 23 - DMA Transfer Enabled"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bits 28:31 - TSICH"]
    #[inline(always)]
    pub fn tsich(&mut self) -> TSICH_W {
        TSICH_W { w: self }
    }
}
