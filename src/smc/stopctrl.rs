#[doc = "Reader of register STOPCTRL"]
pub type R = crate::R<u8, super::STOPCTRL>;
#[doc = "Writer for register STOPCTRL"]
pub type W = crate::W<u8, super::STOPCTRL>;
#[doc = "Register STOPCTRL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::STOPCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "LLS or VLLS Mode Control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLSM_A {
    #[doc = "0: VLLS0 if PMCTRL\\[STOPM\\]=VLLSx, reserved if PMCTRL\\[STOPM\\]=LLSx"]
    _000,
    #[doc = "1: VLLS1 if PMCTRL\\[STOPM\\]=VLLSx, reserved if PMCTRL\\[STOPM\\]=LLSx"]
    _001,
    #[doc = "2: VLLS2 if PMCTRL\\[STOPM\\]=VLLSx, LLS2 if PMCTRL\\[STOPM\\]=LLSx"]
    _010,
    #[doc = "3: VLLS3 if PMCTRL\\[STOPM\\]=VLLSx, LLS3 if PMCTRL\\[STOPM\\]=LLSx"]
    _011,
}
impl From<LLSM_A> for u8 {
    #[inline(always)]
    fn from(variant: LLSM_A) -> Self {
        match variant {
            LLSM_A::_000 => 0,
            LLSM_A::_001 => 1,
            LLSM_A::_010 => 2,
            LLSM_A::_011 => 3,
        }
    }
}
#[doc = "Reader of field `LLSM`"]
pub type LLSM_R = crate::R<u8, LLSM_A>;
impl LLSM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LLSM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LLSM_A::_000),
            1 => Val(LLSM_A::_001),
            2 => Val(LLSM_A::_010),
            3 => Val(LLSM_A::_011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == LLSM_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == LLSM_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == LLSM_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == LLSM_A::_011
    }
}
#[doc = "Write proxy for field `LLSM`"]
pub struct LLSM_W<'a> {
    w: &'a mut W,
}
impl<'a> LLSM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LLSM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VLLS0 if PMCTRL\\[STOPM\\]=VLLSx, reserved if PMCTRL\\[STOPM\\]=LLSx"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(LLSM_A::_000)
    }
    #[doc = "VLLS1 if PMCTRL\\[STOPM\\]=VLLSx, reserved if PMCTRL\\[STOPM\\]=LLSx"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(LLSM_A::_001)
    }
    #[doc = "VLLS2 if PMCTRL\\[STOPM\\]=VLLSx, LLS2 if PMCTRL\\[STOPM\\]=LLSx"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(LLSM_A::_010)
    }
    #[doc = "VLLS3 if PMCTRL\\[STOPM\\]=VLLSx, LLS3 if PMCTRL\\[STOPM\\]=LLSx"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(LLSM_A::_011)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
#[doc = "RAM2 Power Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM2PO_A {
    #[doc = "0: RAM2 not powered in LLS2/VLLS2"]
    _0,
    #[doc = "1: RAM2 powered in LLS2/VLLS2"]
    _1,
}
impl From<RAM2PO_A> for bool {
    #[inline(always)]
    fn from(variant: RAM2PO_A) -> Self {
        match variant {
            RAM2PO_A::_0 => false,
            RAM2PO_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RAM2PO`"]
pub type RAM2PO_R = crate::R<bool, RAM2PO_A>;
impl RAM2PO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM2PO_A {
        match self.bits {
            false => RAM2PO_A::_0,
            true => RAM2PO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RAM2PO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RAM2PO_A::_1
    }
}
#[doc = "Write proxy for field `RAM2PO`"]
pub struct RAM2PO_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM2PO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM2PO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RAM2 not powered in LLS2/VLLS2"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RAM2PO_A::_0)
    }
    #[doc = "RAM2 powered in LLS2/VLLS2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RAM2PO_A::_1)
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
#[doc = "POR Power Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORPO_A {
    #[doc = "0: POR detect circuit is enabled in VLLS0"]
    _0,
    #[doc = "1: POR detect circuit is disabled in VLLS0"]
    _1,
}
impl From<PORPO_A> for bool {
    #[inline(always)]
    fn from(variant: PORPO_A) -> Self {
        match variant {
            PORPO_A::_0 => false,
            PORPO_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PORPO`"]
pub type PORPO_R = crate::R<bool, PORPO_A>;
impl PORPO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORPO_A {
        match self.bits {
            false => PORPO_A::_0,
            true => PORPO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORPO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORPO_A::_1
    }
}
#[doc = "Write proxy for field `PORPO`"]
pub struct PORPO_W<'a> {
    w: &'a mut W,
}
impl<'a> PORPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORPO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "POR detect circuit is enabled in VLLS0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORPO_A::_0)
    }
    #[doc = "POR detect circuit is disabled in VLLS0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORPO_A::_1)
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
#[doc = "Partial Stop Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSTOPO_A {
    #[doc = "0: STOP - Normal Stop mode"]
    _00,
    #[doc = "1: PSTOP1 - Partial Stop with both system and bus clocks disabled"]
    _01,
    #[doc = "2: PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
    _10,
}
impl From<PSTOPO_A> for u8 {
    #[inline(always)]
    fn from(variant: PSTOPO_A) -> Self {
        match variant {
            PSTOPO_A::_00 => 0,
            PSTOPO_A::_01 => 1,
            PSTOPO_A::_10 => 2,
        }
    }
}
#[doc = "Reader of field `PSTOPO`"]
pub type PSTOPO_R = crate::R<u8, PSTOPO_A>;
impl PSTOPO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PSTOPO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PSTOPO_A::_00),
            1 => Val(PSTOPO_A::_01),
            2 => Val(PSTOPO_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PSTOPO_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PSTOPO_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PSTOPO_A::_10
    }
}
#[doc = "Write proxy for field `PSTOPO`"]
pub struct PSTOPO_W<'a> {
    w: &'a mut W,
}
impl<'a> PSTOPO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSTOPO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "STOP - Normal Stop mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PSTOPO_A::_00)
    }
    #[doc = "PSTOP1 - Partial Stop with both system and bus clocks disabled"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PSTOPO_A::_01)
    }
    #[doc = "PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PSTOPO_A::_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u8) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - LLS or VLLS Mode Control"]
    #[inline(always)]
    pub fn llsm(&self) -> LLSM_R {
        LLSM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - RAM2 Power Option"]
    #[inline(always)]
    pub fn ram2po(&self) -> RAM2PO_R {
        RAM2PO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline(always)]
    pub fn porpo(&self) -> PORPO_R {
        PORPO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Partial Stop Option"]
    #[inline(always)]
    pub fn pstopo(&self) -> PSTOPO_R {
        PSTOPO_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LLS or VLLS Mode Control"]
    #[inline(always)]
    pub fn llsm(&mut self) -> LLSM_W {
        LLSM_W { w: self }
    }
    #[doc = "Bit 4 - RAM2 Power Option"]
    #[inline(always)]
    pub fn ram2po(&mut self) -> RAM2PO_W {
        RAM2PO_W { w: self }
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline(always)]
    pub fn porpo(&mut self) -> PORPO_W {
        PORPO_W { w: self }
    }
    #[doc = "Bits 6:7 - Partial Stop Option"]
    #[inline(always)]
    pub fn pstopo(&mut self) -> PSTOPO_W {
        PSTOPO_W { w: self }
    }
}
