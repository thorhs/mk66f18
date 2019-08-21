#[doc = "Reader of register AC%s"]
pub type R = crate::R<u32, super::AC>;
#[doc = "Writer for register AC%s"]
pub type W = crate::W<u32, super::AC>;
#[doc = "Register AC%s `reset()`'s with value 0"]
impl crate::ResetValue for super::AC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Initiate precharge all (pall) command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IP_A {
    #[doc = "0: Take no action."]
    _0,
    #[doc = "1: A pall command is sent to the associated SDRAM block. During initialization, this command is executed after all DRAM controller registers are programmed. After IP is set, the next write to an appropriate SDRAM address generates the pall command to the SDRAM block."]
    _1,
}
impl From<IP_A> for bool {
    #[inline(always)]
    fn from(variant: IP_A) -> Self {
        match variant {
            IP_A::_0 => false,
            IP_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IP`"]
pub type IP_R = crate::R<bool, IP_A>;
impl IP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IP_A {
        match self.bits {
            false => IP_A::_0,
            true => IP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IP_A::_1
    }
}
#[doc = "Write proxy for field `IP`"]
pub struct IP_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Take no action."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IP_A::_0)
    }
    #[doc = "A pall command is sent to the associated SDRAM block. During initialization, this command is executed after all DRAM controller registers are programmed. After IP is set, the next write to an appropriate SDRAM address generates the pall command to the SDRAM block."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IP_A::_1)
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
#[doc = "Port size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
    #[doc = "0: 32-bit port"]
    _00,
    #[doc = "1: 8-bit port"]
    _01,
    #[doc = "2: 16-bit port"]
    _10,
    #[doc = "3: 16-bit port"]
    _11,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        match variant {
            PS_A::_00 => 0,
            PS_A::_01 => 1,
            PS_A::_10 => 2,
            PS_A::_11 => 3,
        }
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u8, PS_A>;
impl PS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::_00,
            1 => PS_A::_01,
            2 => PS_A::_10,
            3 => PS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PS_A::_11
    }
}
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32-bit port"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PS_A::_00)
    }
    #[doc = "8-bit port"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PS_A::_01)
    }
    #[doc = "16-bit port"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PS_A::_10)
    }
    #[doc = "16-bit port"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Initiate mode register set (mrs) command.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMRS_A {
    #[doc = "0: Take no action"]
    _0,
    #[doc = "1: Initiate mrs command"]
    _1,
}
impl From<IMRS_A> for bool {
    #[inline(always)]
    fn from(variant: IMRS_A) -> Self {
        match variant {
            IMRS_A::_0 => false,
            IMRS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `IMRS`"]
pub type IMRS_R = crate::R<bool, IMRS_A>;
impl IMRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMRS_A {
        match self.bits {
            false => IMRS_A::_0,
            true => IMRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IMRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IMRS_A::_1
    }
}
#[doc = "Write proxy for field `IMRS`"]
pub struct IMRS_W<'a> {
    w: &'a mut W,
}
impl<'a> IMRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMRS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Take no action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IMRS_A::_0)
    }
    #[doc = "Initiate mrs command"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IMRS_A::_1)
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
#[doc = "Reader of field `CBM`"]
pub type CBM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CBM`"]
pub struct CBM_W<'a> {
    w: &'a mut W,
}
impl<'a> CBM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `CASL`"]
pub type CASL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CASL`"]
pub struct CASL_W<'a> {
    w: &'a mut W,
}
impl<'a> CASL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Refresh enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_A {
    #[doc = "0: Do not refresh associated DRAM block"]
    _0,
    #[doc = "1: Refresh associated DRAM block"]
    _1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        match variant {
            RE_A::_0 => false,
            RE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RE`"]
pub type RE_R = crate::R<bool, RE_A>;
impl RE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::_0,
            true => RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RE_A::_1
    }
}
#[doc = "Write proxy for field `RE`"]
pub struct RE_W<'a> {
    w: &'a mut W,
}
impl<'a> RE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Do not refresh associated DRAM block"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RE_A::_0)
    }
    #[doc = "Refresh associated DRAM block"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RE_A::_1)
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
#[doc = "Reader of field `BA`"]
pub type BA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BA`"]
pub struct BA_W<'a> {
    w: &'a mut W,
}
impl<'a> BA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 18)) | (((value as u32) & 0x3fff) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Initiate precharge all (pall) command."]
    #[inline(always)]
    pub fn ip(&self) -> IP_R {
        IP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Port size."]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Initiate mode register set (mrs) command."]
    #[inline(always)]
    pub fn imrs(&self) -> IMRS_R {
        IMRS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Command bit location"]
    #[inline(always)]
    pub fn cbm(&self) -> CBM_R {
        CBM_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - CAS Latency"]
    #[inline(always)]
    pub fn casl(&self) -> CASL_R {
        CASL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Refresh enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 18:31 - Base address register."]
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 3 - Initiate precharge all (pall) command."]
    #[inline(always)]
    pub fn ip(&mut self) -> IP_W {
        IP_W { w: self }
    }
    #[doc = "Bits 4:5 - Port size."]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 6 - Initiate mode register set (mrs) command."]
    #[inline(always)]
    pub fn imrs(&mut self) -> IMRS_W {
        IMRS_W { w: self }
    }
    #[doc = "Bits 8:10 - Command bit location"]
    #[inline(always)]
    pub fn cbm(&mut self) -> CBM_W {
        CBM_W { w: self }
    }
    #[doc = "Bits 12:13 - CAS Latency"]
    #[inline(always)]
    pub fn casl(&mut self) -> CASL_W {
        CASL_W { w: self }
    }
    #[doc = "Bit 15 - Refresh enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Bits 18:31 - Base address register."]
    #[inline(always)]
    pub fn ba(&mut self) -> BA_W {
        BA_W { w: self }
    }
}
