#[doc = "Reader of register S"]
pub type R = crate::R<u8, super::S>;
#[doc = "Writer for register S"]
pub type W = crate::W<u8, super::S>;
#[doc = "Register S `reset()`'s with value 0x80"]
impl crate::ResetValue for super::S {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Receive Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXAK_A {
    #[doc = "0: Acknowledge signal was received after the completion of one byte of data transmission on the bus"]
    _0,
    #[doc = "1: No acknowledge signal detected"]
    _1,
}
impl From<RXAK_A> for bool {
    #[inline(always)]
    fn from(variant: RXAK_A) -> Self {
        match variant {
            RXAK_A::_0 => false,
            RXAK_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RXAK`"]
pub type RXAK_R = crate::R<bool, RXAK_A>;
impl RXAK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXAK_A {
        match self.bits {
            false => RXAK_A::_0,
            true => RXAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXAK_A::_1
    }
}
#[doc = "Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IICIF_A {
    #[doc = "0: No interrupt pending"]
    _0,
    #[doc = "1: Interrupt pending"]
    _1,
}
impl From<IICIF_A> for bool {
    #[inline(always)]
    fn from(variant: IICIF_A) -> Self {
        match variant {
            IICIF_A::_0 => false,
            IICIF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IICIF`"]
pub type IICIF_R = crate::R<bool, IICIF_A>;
impl IICIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICIF_A {
        match self.bits {
            false => IICIF_A::_0,
            true => IICIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICIF_A::_1
    }
}
#[doc = "Write proxy for field `IICIF`"]
pub struct IICIF_W<'a> {
    w: &'a mut W,
}
impl<'a> IICIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IICIF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICIF_A::_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICIF_A::_1)
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
#[doc = "Slave Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRW_A {
    #[doc = "0: Slave receive, master writing to slave"]
    _0,
    #[doc = "1: Slave transmit, master reading from slave"]
    _1,
}
impl From<SRW_A> for bool {
    #[inline(always)]
    fn from(variant: SRW_A) -> Self {
        match variant {
            SRW_A::_0 => false,
            SRW_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SRW`"]
pub type SRW_R = crate::R<bool, SRW_A>;
impl SRW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRW_A {
        match self.bits {
            false => SRW_A::_0,
            true => SRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRW_A::_1
    }
}
#[doc = "Range Address Match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM_A {
    #[doc = "0: Not addressed"]
    _0,
    #[doc = "1: Addressed as a slave"]
    _1,
}
impl From<RAM_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_A) -> Self {
        match variant {
            RAM_A::_0 => false,
            RAM_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RAM`"]
pub type RAM_R = crate::R<bool, RAM_A>;
impl RAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_A {
        match self.bits {
            false => RAM_A::_0,
            true => RAM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RAM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RAM_A::_1
    }
}
#[doc = "Write proxy for field `RAM`"]
pub struct RAM_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not addressed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RAM_A::_0)
    }
    #[doc = "Addressed as a slave"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RAM_A::_1)
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
#[doc = "Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBL_A {
    #[doc = "0: Standard bus operation."]
    _0,
    #[doc = "1: Loss of arbitration."]
    _1,
}
impl From<ARBL_A> for bool {
    #[inline(always)]
    fn from(variant: ARBL_A) -> Self {
        match variant {
            ARBL_A::_0 => false,
            ARBL_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ARBL`"]
pub type ARBL_R = crate::R<bool, ARBL_A>;
impl ARBL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBL_A {
        match self.bits {
            false => ARBL_A::_0,
            true => ARBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ARBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ARBL_A::_1
    }
}
#[doc = "Write proxy for field `ARBL`"]
pub struct ARBL_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Standard bus operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ARBL_A::_0)
    }
    #[doc = "Loss of arbitration."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ARBL_A::_1)
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
#[doc = "Bus Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: Bus is idle"]
    _0,
    #[doc = "1: Bus is busy"]
    _1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        match variant {
            BUSY_A::_0 => false,
            BUSY_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::_0,
            true => BUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSY_A::_1
    }
}
#[doc = "Addressed As A Slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IAAS_A {
    #[doc = "0: Not addressed"]
    _0,
    #[doc = "1: Addressed as a slave"]
    _1,
}
impl From<IAAS_A> for bool {
    #[inline(always)]
    fn from(variant: IAAS_A) -> Self {
        match variant {
            IAAS_A::_0 => false,
            IAAS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IAAS`"]
pub type IAAS_R = crate::R<bool, IAAS_A>;
impl IAAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IAAS_A {
        match self.bits {
            false => IAAS_A::_0,
            true => IAAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IAAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IAAS_A::_1
    }
}
#[doc = "Write proxy for field `IAAS`"]
pub struct IAAS_W<'a> {
    w: &'a mut W,
}
impl<'a> IAAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IAAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not addressed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IAAS_A::_0)
    }
    #[doc = "Addressed as a slave"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IAAS_A::_1)
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
#[doc = "Transfer Complete Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCF_A {
    #[doc = "0: Transfer in progress"]
    _0,
    #[doc = "1: Transfer complete"]
    _1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        match variant {
            TCF_A::_0 => false,
            TCF_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TCF`"]
pub type TCF_R = crate::R<bool, TCF_A>;
impl TCF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::_0,
            true => TCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Receive Acknowledge"]
    #[inline(always)]
    pub fn rxak(&self) -> RXAK_R {
        RXAK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt Flag"]
    #[inline(always)]
    pub fn iicif(&self) -> IICIF_R {
        IICIF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slave Read/Write"]
    #[inline(always)]
    pub fn srw(&self) -> SRW_R {
        SRW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Range Address Match"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    pub fn arbl(&self) -> ARBL_R {
        ARBL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bus Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Addressed As A Slave"]
    #[inline(always)]
    pub fn iaas(&self) -> IAAS_R {
        IAAS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt Flag"]
    #[inline(always)]
    pub fn iicif(&mut self) -> IICIF_W {
        IICIF_W { w: self }
    }
    #[doc = "Bit 3 - Range Address Match"]
    #[inline(always)]
    pub fn ram(&mut self) -> RAM_W {
        RAM_W { w: self }
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    pub fn arbl(&mut self) -> ARBL_W {
        ARBL_W { w: self }
    }
    #[doc = "Bit 6 - Addressed As A Slave"]
    #[inline(always)]
    pub fn iaas(&mut self) -> IAAS_W {
        IAAS_W { w: self }
    }
}
