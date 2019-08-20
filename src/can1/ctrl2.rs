#[doc = "Reader of register CTRL2"]
pub type R = crate::R<u32, super::CTRL2>;
#[doc = "Writer for register CTRL2"]
pub type W = crate::W<u32, super::CTRL2>;
#[doc = "Register CTRL2 `reset()`'s with value 0x00b0_0000"]
impl crate::ResetValue for super::CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00b0_0000
    }
}
#[doc = "Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EACEN_A {
    #[doc = "0: Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    _0,
    #[doc = "1: Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    _1,
}
impl From<EACEN_A> for bool {
    #[inline(always)]
    fn from(variant: EACEN_A) -> Self {
        match variant {
            EACEN_A::_0 => false,
            EACEN_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `EACEN`"]
pub type EACEN_R = crate::R<bool, EACEN_A>;
impl EACEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EACEN_A {
        match self.bits {
            false => EACEN_A::_0,
            true => EACEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EACEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EACEN_A::_1
    }
}
#[doc = "Write proxy for field `EACEN`"]
pub struct EACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EACEN_A::_0)
    }
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EACEN_A::_1)
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
#[doc = "Remote Request Storing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRS_A {
    #[doc = "0: Remote Response Frame is generated."]
    _0,
    #[doc = "1: Remote Request Frame is stored."]
    _1,
}
impl From<RRS_A> for bool {
    #[inline(always)]
    fn from(variant: RRS_A) -> Self {
        match variant {
            RRS_A::_0 => false,
            RRS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RRS`"]
pub type RRS_R = crate::R<bool, RRS_A>;
impl RRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRS_A {
        match self.bits {
            false => RRS_A::_0,
            true => RRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RRS_A::_1
    }
}
#[doc = "Write proxy for field `RRS`"]
pub struct RRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Remote Response Frame is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RRS_A::_0)
    }
    #[doc = "Remote Request Frame is stored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RRS_A::_1)
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
#[doc = "Mailboxes Reception Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRP_A {
    #[doc = "0: Matching starts from Rx FIFO and continues on Mailboxes."]
    _0,
    #[doc = "1: Matching starts from Mailboxes and continues on Rx FIFO."]
    _1,
}
impl From<MRP_A> for bool {
    #[inline(always)]
    fn from(variant: MRP_A) -> Self {
        match variant {
            MRP_A::_0 => false,
            MRP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `MRP`"]
pub type MRP_R = crate::R<bool, MRP_A>;
impl MRP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRP_A {
        match self.bits {
            false => MRP_A::_0,
            true => MRP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MRP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MRP_A::_1
    }
}
#[doc = "Write proxy for field `MRP`"]
pub struct MRP_W<'a> {
    w: &'a mut W,
}
impl<'a> MRP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MRP_A::_0)
    }
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MRP_A::_1)
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
#[doc = "Reader of field `TASD`"]
pub type TASD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TASD`"]
pub struct TASD_W<'a> {
    w: &'a mut W,
}
impl<'a> TASD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | (((value as u32) & 0x1f) << 19);
        self.w
    }
}
#[doc = "Reader of field `RFFN`"]
pub type RFFN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFFN`"]
pub struct RFFN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFFN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Write-Access To Memory In Freeze Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRMFRZ_A {
    #[doc = "0: Maintain the write access restrictions."]
    _0,
    #[doc = "1: Enable unrestricted write access to FlexCAN memory."]
    _1,
}
impl From<WRMFRZ_A> for bool {
    #[inline(always)]
    fn from(variant: WRMFRZ_A) -> Self {
        match variant {
            WRMFRZ_A::_0 => false,
            WRMFRZ_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WRMFRZ`"]
pub type WRMFRZ_R = crate::R<bool, WRMFRZ_A>;
impl WRMFRZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRMFRZ_A {
        match self.bits {
            false => WRMFRZ_A::_0,
            true => WRMFRZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WRMFRZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WRMFRZ_A::_1
    }
}
#[doc = "Write proxy for field `WRMFRZ`"]
pub struct WRMFRZ_W<'a> {
    w: &'a mut W,
}
impl<'a> WRMFRZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRMFRZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Maintain the write access restrictions."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WRMFRZ_A::_0)
    }
    #[doc = "Enable unrestricted write access to FlexCAN memory."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WRMFRZ_A::_1)
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
impl R {
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline(always)]
    pub fn eacen(&self) -> EACEN_R {
        EACEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline(always)]
    pub fn mrp(&self) -> MRP_R {
        MRP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline(always)]
    pub fn tasd(&self) -> TASD_R {
        TASD_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - Number Of Rx FIFO Filters"]
    #[inline(always)]
    pub fn rffn(&self) -> RFFN_R {
        RFFN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Write-Access To Memory In Freeze Mode"]
    #[inline(always)]
    pub fn wrmfrz(&self) -> WRMFRZ_R {
        WRMFRZ_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline(always)]
    pub fn eacen(&mut self) -> EACEN_W {
        EACEN_W { w: self }
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline(always)]
    pub fn rrs(&mut self) -> RRS_W {
        RRS_W { w: self }
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline(always)]
    pub fn mrp(&mut self) -> MRP_W {
        MRP_W { w: self }
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline(always)]
    pub fn tasd(&mut self) -> TASD_W {
        TASD_W { w: self }
    }
    #[doc = "Bits 24:27 - Number Of Rx FIFO Filters"]
    #[inline(always)]
    pub fn rffn(&mut self) -> RFFN_W {
        RFFN_W { w: self }
    }
    #[doc = "Bit 28 - Write-Access To Memory In Freeze Mode"]
    #[inline(always)]
    pub fn wrmfrz(&mut self) -> WRMFRZ_W {
        WRMFRZ_W { w: self }
    }
}
