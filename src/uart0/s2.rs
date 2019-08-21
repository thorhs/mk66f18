#[doc = "Reader of register S2"]
pub type R = crate::R<u8, super::S2>;
#[doc = "Writer for register S2"]
pub type W = crate::W<u8, super::S2>;
#[doc = "Register S2 `reset()`'s with value 0"]
impl crate::ResetValue for super::S2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receiver Active Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAF_A {
    #[doc = "0: UART receiver idle/inactive waiting for a start bit."]
    _0,
    #[doc = "1: UART receiver active, RxD input not idle."]
    _1,
}
impl From<RAF_A> for bool {
    #[inline(always)]
    fn from(variant: RAF_A) -> Self {
        match variant {
            RAF_A::_0 => false,
            RAF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RAF`"]
pub type RAF_R = crate::R<bool, RAF_A>;
impl RAF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAF_A {
        match self.bits {
            false => RAF_A::_0,
            true => RAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RAF_A::_1
    }
}
#[doc = "LIN Break Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDE_A {
    #[doc = "0: Break character detection is disabled."]
    _0,
    #[doc = "1: Break character is detected at length of 11 bit times if C1\\[M\\] = 0 or 12 bits time if C1\\[M\\] = 1."]
    _1,
}
impl From<LBKDE_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDE_A) -> Self {
        match variant {
            LBKDE_A::_0 => false,
            LBKDE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LBKDE`"]
pub type LBKDE_R = crate::R<bool, LBKDE_A>;
impl LBKDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDE_A {
        match self.bits {
            false => LBKDE_A::_0,
            true => LBKDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LBKDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LBKDE_A::_1
    }
}
#[doc = "Write proxy for field `LBKDE`"]
pub struct LBKDE_W<'a> {
    w: &'a mut W,
}
impl<'a> LBKDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBKDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Break character detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDE_A::_0)
    }
    #[doc = "Break character is detected at length of 11 bit times if C1\\[M\\] = 0 or 12 bits time if C1\\[M\\] = 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDE_A::_1)
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
#[doc = "Break Transmit Character Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRK13_A {
    #[doc = "0: Break character is 10, 11, or 12 bits long."]
    _0,
    #[doc = "1: Break character is 13 or 14 bits long."]
    _1,
}
impl From<BRK13_A> for bool {
    #[inline(always)]
    fn from(variant: BRK13_A) -> Self {
        match variant {
            BRK13_A::_0 => false,
            BRK13_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BRK13`"]
pub type BRK13_R = crate::R<bool, BRK13_A>;
impl BRK13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRK13_A {
        match self.bits {
            false => BRK13_A::_0,
            true => BRK13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRK13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRK13_A::_1
    }
}
#[doc = "Write proxy for field `BRK13`"]
pub struct BRK13_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRK13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Break character is 10, 11, or 12 bits long."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRK13_A::_0)
    }
    #[doc = "Break character is 13 or 14 bits long."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRK13_A::_1)
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
#[doc = "Receive Wakeup Idle Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWUID_A {
    #[doc = "0: S1\\[IDLE\\] is not set upon detection of an idle character."]
    _0,
    #[doc = "1: S1\\[IDLE\\] is set upon detection of an idle character."]
    _1,
}
impl From<RWUID_A> for bool {
    #[inline(always)]
    fn from(variant: RWUID_A) -> Self {
        match variant {
            RWUID_A::_0 => false,
            RWUID_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWUID`"]
pub type RWUID_R = crate::R<bool, RWUID_A>;
impl RWUID_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWUID_A {
        match self.bits {
            false => RWUID_A::_0,
            true => RWUID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWUID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWUID_A::_1
    }
}
#[doc = "Write proxy for field `RWUID`"]
pub struct RWUID_W<'a> {
    w: &'a mut W,
}
impl<'a> RWUID_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWUID_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "S1\\[IDLE\\] is not set upon detection of an idle character."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWUID_A::_0)
    }
    #[doc = "S1\\[IDLE\\] is set upon detection of an idle character."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWUID_A::_1)
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
#[doc = "Receive Data Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINV_A {
    #[doc = "0: Receive data is not inverted."]
    _0,
    #[doc = "1: Receive data is inverted."]
    _1,
}
impl From<RXINV_A> for bool {
    #[inline(always)]
    fn from(variant: RXINV_A) -> Self {
        match variant {
            RXINV_A::_0 => false,
            RXINV_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXINV`"]
pub type RXINV_R = crate::R<bool, RXINV_A>;
impl RXINV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXINV_A {
        match self.bits {
            false => RXINV_A::_0,
            true => RXINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXINV_A::_1
    }
}
#[doc = "Write proxy for field `RXINV`"]
pub struct RXINV_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXINV_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive data is not inverted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXINV_A::_0)
    }
    #[doc = "Receive data is inverted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXINV_A::_1)
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
#[doc = "Most Significant Bit First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBF_A {
    #[doc = "0: LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    _0,
    #[doc = "1: MSB (bit8, bit7 or bit6) is the first bit that is transmitted following the start bit, depending on the setting of C1\\[M\\] and C1\\[PE\\]. Further, the first bit received after the start bit is identified as bit8, bit7, or bit6, depending on the setting of C1\\[M\\] and C1\\[PE\\]."]
    _1,
}
impl From<MSBF_A> for bool {
    #[inline(always)]
    fn from(variant: MSBF_A) -> Self {
        match variant {
            MSBF_A::_0 => false,
            MSBF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MSBF`"]
pub type MSBF_R = crate::R<bool, MSBF_A>;
impl MSBF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBF_A {
        match self.bits {
            false => MSBF_A::_0,
            true => MSBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSBF_A::_1
    }
}
#[doc = "Write proxy for field `MSBF`"]
pub struct MSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> MSBF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSBF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSBF_A::_0)
    }
    #[doc = "MSB (bit8, bit7 or bit6) is the first bit that is transmitted following the start bit, depending on the setting of C1\\[M\\] and C1\\[PE\\]. Further, the first bit received after the start bit is identified as bit8, bit7, or bit6, depending on the setting of C1\\[M\\] and C1\\[PE\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSBF_A::_1)
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
#[doc = "RxD Pin Active Edge Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIF_A {
    #[doc = "0: No active edge on the receive pin has occurred."]
    _0,
    #[doc = "1: An active edge on the receive pin has occurred."]
    _1,
}
impl From<RXEDGIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXEDGIF_A) -> Self {
        match variant {
            RXEDGIF_A::_0 => false,
            RXEDGIF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXEDGIF`"]
pub type RXEDGIF_R = crate::R<bool, RXEDGIF_A>;
impl RXEDGIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEDGIF_A {
        match self.bits {
            false => RXEDGIF_A::_0,
            true => RXEDGIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXEDGIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXEDGIF_A::_1
    }
}
#[doc = "Write proxy for field `RXEDGIF`"]
pub struct RXEDGIF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEDGIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXEDGIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEDGIF_A::_0)
    }
    #[doc = "An active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEDGIF_A::_1)
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
#[doc = "LIN Break Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIF_A {
    #[doc = "0: No LIN break character detected."]
    _0,
    #[doc = "1: LIN break character detected."]
    _1,
}
impl From<LBKDIF_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDIF_A) -> Self {
        match variant {
            LBKDIF_A::_0 => false,
            LBKDIF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LBKDIF`"]
pub type LBKDIF_R = crate::R<bool, LBKDIF_A>;
impl LBKDIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDIF_A {
        match self.bits {
            false => LBKDIF_A::_0,
            true => LBKDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LBKDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LBKDIF_A::_1
    }
}
#[doc = "Write proxy for field `LBKDIF`"]
pub struct LBKDIF_W<'a> {
    w: &'a mut W,
}
impl<'a> LBKDIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LBKDIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No LIN break character detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDIF_A::_0)
    }
    #[doc = "LIN break character detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDIF_A::_1)
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
    #[doc = "Bit 0 - Receiver Active Flag"]
    #[inline(always)]
    pub fn raf(&self) -> RAF_R {
        RAF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LIN Break Detection Enable"]
    #[inline(always)]
    pub fn lbkde(&self) -> LBKDE_R {
        LBKDE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Break Transmit Character Length"]
    #[inline(always)]
    pub fn brk13(&self) -> BRK13_R {
        BRK13_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive Wakeup Idle Detect"]
    #[inline(always)]
    pub fn rwuid(&self) -> RWUID_R {
        RWUID_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Data Inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RxD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub fn rxedgif(&self) -> RXEDGIF_R {
        RXEDGIF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    pub fn lbkdif(&self) -> LBKDIF_R {
        LBKDIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - LIN Break Detection Enable"]
    #[inline(always)]
    pub fn lbkde(&mut self) -> LBKDE_W {
        LBKDE_W { w: self }
    }
    #[doc = "Bit 2 - Break Transmit Character Length"]
    #[inline(always)]
    pub fn brk13(&mut self) -> BRK13_W {
        BRK13_W { w: self }
    }
    #[doc = "Bit 3 - Receive Wakeup Idle Detect"]
    #[inline(always)]
    pub fn rwuid(&mut self) -> RWUID_W {
        RWUID_W { w: self }
    }
    #[doc = "Bit 4 - Receive Data Inversion"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W {
        RXINV_W { w: self }
    }
    #[doc = "Bit 5 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&mut self) -> MSBF_W {
        MSBF_W { w: self }
    }
    #[doc = "Bit 6 - RxD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub fn rxedgif(&mut self) -> RXEDGIF_W {
        RXEDGIF_W { w: self }
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    pub fn lbkdif(&mut self) -> LBKDIF_W {
        LBKDIF_W { w: self }
    }
}
