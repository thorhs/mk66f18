#[doc = "Reader of register SMB"]
pub type R = crate::R<u8, super::SMB>;
#[doc = "Writer for register SMB"]
pub type W = crate::W<u8, super::SMB>;
#[doc = "Register SMB `reset()`'s with value 0"]
impl crate::ResetValue for super::SMB {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SHTF2 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHTF2IE_A {
    #[doc = "0: SHTF2 interrupt is disabled"]
    _0,
    #[doc = "1: SHTF2 interrupt is enabled"]
    _1,
}
impl From<SHTF2IE_A> for bool {
    #[inline(always)]
    fn from(variant: SHTF2IE_A) -> Self {
        match variant {
            SHTF2IE_A::_0 => false,
            SHTF2IE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SHTF2IE`"]
pub type SHTF2IE_R = crate::R<bool, SHTF2IE_A>;
impl SHTF2IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHTF2IE_A {
        match self.bits {
            false => SHTF2IE_A::_0,
            true => SHTF2IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHTF2IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHTF2IE_A::_1
    }
}
#[doc = "Write proxy for field `SHTF2IE`"]
pub struct SHTF2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SHTF2IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHTF2IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SHTF2 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHTF2IE_A::_0)
    }
    #[doc = "SHTF2 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHTF2IE_A::_1)
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
#[doc = "SCL High Timeout Flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHTF2_A {
    #[doc = "0: No SCL high and SDA low timeout occurs"]
    _0,
    #[doc = "1: SCL high and SDA low timeout occurs"]
    _1,
}
impl From<SHTF2_A> for bool {
    #[inline(always)]
    fn from(variant: SHTF2_A) -> Self {
        match variant {
            SHTF2_A::_0 => false,
            SHTF2_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SHTF2`"]
pub type SHTF2_R = crate::R<bool, SHTF2_A>;
impl SHTF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHTF2_A {
        match self.bits {
            false => SHTF2_A::_0,
            true => SHTF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHTF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHTF2_A::_1
    }
}
#[doc = "Write proxy for field `SHTF2`"]
pub struct SHTF2_W<'a> {
    w: &'a mut W,
}
impl<'a> SHTF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SHTF2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No SCL high and SDA low timeout occurs"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHTF2_A::_0)
    }
    #[doc = "SCL high and SDA low timeout occurs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHTF2_A::_1)
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
#[doc = "SCL High Timeout Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SHTF1_A {
    #[doc = "0: No SCL high and SDA high timeout occurs"]
    _0,
    #[doc = "1: SCL high and SDA high timeout occurs"]
    _1,
}
impl From<SHTF1_A> for bool {
    #[inline(always)]
    fn from(variant: SHTF1_A) -> Self {
        match variant {
            SHTF1_A::_0 => false,
            SHTF1_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SHTF1`"]
pub type SHTF1_R = crate::R<bool, SHTF1_A>;
impl SHTF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHTF1_A {
        match self.bits {
            false => SHTF1_A::_0,
            true => SHTF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHTF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHTF1_A::_1
    }
}
#[doc = "SCL Low Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLTF_A {
    #[doc = "0: No low timeout occurs"]
    _0,
    #[doc = "1: Low timeout occurs"]
    _1,
}
impl From<SLTF_A> for bool {
    #[inline(always)]
    fn from(variant: SLTF_A) -> Self {
        match variant {
            SLTF_A::_0 => false,
            SLTF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SLTF`"]
pub type SLTF_R = crate::R<bool, SLTF_A>;
impl SLTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLTF_A {
        match self.bits {
            false => SLTF_A::_0,
            true => SLTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLTF_A::_1
    }
}
#[doc = "Write proxy for field `SLTF`"]
pub struct SLTF_W<'a> {
    w: &'a mut W,
}
impl<'a> SLTF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLTF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No low timeout occurs"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLTF_A::_0)
    }
    #[doc = "Low timeout occurs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLTF_A::_1)
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
#[doc = "Timeout Counter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCKSEL_A {
    #[doc = "0: Timeout counter counts at the frequency of the I2C module clock / 64"]
    _0,
    #[doc = "1: Timeout counter counts at the frequency of the I2C module clock"]
    _1,
}
impl From<TCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TCKSEL_A) -> Self {
        match variant {
            TCKSEL_A::_0 => false,
            TCKSEL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TCKSEL`"]
pub type TCKSEL_R = crate::R<bool, TCKSEL_A>;
impl TCKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCKSEL_A {
        match self.bits {
            false => TCKSEL_A::_0,
            true => TCKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCKSEL_A::_1
    }
}
#[doc = "Write proxy for field `TCKSEL`"]
pub struct TCKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TCKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timeout counter counts at the frequency of the I2C module clock / 64"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCKSEL_A::_0)
    }
    #[doc = "Timeout counter counts at the frequency of the I2C module clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCKSEL_A::_1)
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
#[doc = "Second I2C Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIICAEN_A {
    #[doc = "0: I2C address register 2 matching is disabled"]
    _0,
    #[doc = "1: I2C address register 2 matching is enabled"]
    _1,
}
impl From<SIICAEN_A> for bool {
    #[inline(always)]
    fn from(variant: SIICAEN_A) -> Self {
        match variant {
            SIICAEN_A::_0 => false,
            SIICAEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SIICAEN`"]
pub type SIICAEN_R = crate::R<bool, SIICAEN_A>;
impl SIICAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIICAEN_A {
        match self.bits {
            false => SIICAEN_A::_0,
            true => SIICAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SIICAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SIICAEN_A::_1
    }
}
#[doc = "Write proxy for field `SIICAEN`"]
pub struct SIICAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIICAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIICAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "I2C address register 2 matching is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIICAEN_A::_0)
    }
    #[doc = "I2C address register 2 matching is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIICAEN_A::_1)
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
#[doc = "SMBus Alert Response Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALERTEN_A {
    #[doc = "0: SMBus alert response address matching is disabled"]
    _0,
    #[doc = "1: SMBus alert response address matching is enabled"]
    _1,
}
impl From<ALERTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ALERTEN_A) -> Self {
        match variant {
            ALERTEN_A::_0 => false,
            ALERTEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ALERTEN`"]
pub type ALERTEN_R = crate::R<bool, ALERTEN_A>;
impl ALERTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALERTEN_A {
        match self.bits {
            false => ALERTEN_A::_0,
            true => ALERTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALERTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALERTEN_A::_1
    }
}
#[doc = "Write proxy for field `ALERTEN`"]
pub struct ALERTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALERTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALERTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SMBus alert response address matching is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALERTEN_A::_0)
    }
    #[doc = "SMBus alert response address matching is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALERTEN_A::_1)
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
#[doc = "Fast NACK/ACK Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FACK_A {
    #[doc = "0: An ACK or NACK is sent on the following receiving data byte"]
    _0,
    #[doc = "1: Writing 0 to TXAK after receiving a data byte generates an ACK. Writing 1 to TXAK after receiving a data byte generates a NACK."]
    _1,
}
impl From<FACK_A> for bool {
    #[inline(always)]
    fn from(variant: FACK_A) -> Self {
        match variant {
            FACK_A::_0 => false,
            FACK_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `FACK`"]
pub type FACK_R = crate::R<bool, FACK_A>;
impl FACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FACK_A {
        match self.bits {
            false => FACK_A::_0,
            true => FACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FACK_A::_1
    }
}
#[doc = "Write proxy for field `FACK`"]
pub struct FACK_W<'a> {
    w: &'a mut W,
}
impl<'a> FACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "An ACK or NACK is sent on the following receiving data byte"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FACK_A::_0)
    }
    #[doc = "Writing 0 to TXAK after receiving a data byte generates an ACK. Writing 1 to TXAK after receiving a data byte generates a NACK."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FACK_A::_1)
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
    #[doc = "Bit 0 - SHTF2 Interrupt Enable"]
    #[inline(always)]
    pub fn shtf2ie(&self) -> SHTF2IE_R {
        SHTF2IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SCL High Timeout Flag 2"]
    #[inline(always)]
    pub fn shtf2(&self) -> SHTF2_R {
        SHTF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SCL High Timeout Flag 1"]
    #[inline(always)]
    pub fn shtf1(&self) -> SHTF1_R {
        SHTF1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SCL Low Timeout Flag"]
    #[inline(always)]
    pub fn sltf(&self) -> SLTF_R {
        SLTF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Timeout Counter Clock Select"]
    #[inline(always)]
    pub fn tcksel(&self) -> TCKSEL_R {
        TCKSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Second I2C Address Enable"]
    #[inline(always)]
    pub fn siicaen(&self) -> SIICAEN_R {
        SIICAEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SMBus Alert Response Address Enable"]
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast NACK/ACK Enable"]
    #[inline(always)]
    pub fn fack(&self) -> FACK_R {
        FACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SHTF2 Interrupt Enable"]
    #[inline(always)]
    pub fn shtf2ie(&mut self) -> SHTF2IE_W {
        SHTF2IE_W { w: self }
    }
    #[doc = "Bit 1 - SCL High Timeout Flag 2"]
    #[inline(always)]
    pub fn shtf2(&mut self) -> SHTF2_W {
        SHTF2_W { w: self }
    }
    #[doc = "Bit 3 - SCL Low Timeout Flag"]
    #[inline(always)]
    pub fn sltf(&mut self) -> SLTF_W {
        SLTF_W { w: self }
    }
    #[doc = "Bit 4 - Timeout Counter Clock Select"]
    #[inline(always)]
    pub fn tcksel(&mut self) -> TCKSEL_W {
        TCKSEL_W { w: self }
    }
    #[doc = "Bit 5 - Second I2C Address Enable"]
    #[inline(always)]
    pub fn siicaen(&mut self) -> SIICAEN_W {
        SIICAEN_W { w: self }
    }
    #[doc = "Bit 6 - SMBus Alert Response Address Enable"]
    #[inline(always)]
    pub fn alerten(&mut self) -> ALERTEN_W {
        ALERTEN_W { w: self }
    }
    #[doc = "Bit 7 - Fast NACK/ACK Enable"]
    #[inline(always)]
    pub fn fack(&mut self) -> FACK_W {
        FACK_W { w: self }
    }
}
