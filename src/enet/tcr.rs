#[doc = "Reader of register TCR"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GTS`"]
pub type GTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTS`"]
pub struct GTS_W<'a> {
    w: &'a mut W,
}
impl<'a> GTS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `FDEN`"]
pub type FDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FDEN`"]
pub struct FDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Transmit Frame Control Pause\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFC_PAUSE_A {
    #[doc = "0: No PAUSE frame transmitted."]
    _0,
    #[doc = "1: The MAC stops transmission of data frames after the current transmission is complete."]
    _1,
}
impl From<TFC_PAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: TFC_PAUSE_A) -> Self {
        match variant {
            TFC_PAUSE_A::_0 => false,
            TFC_PAUSE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TFC_PAUSE`"]
pub type TFC_PAUSE_R = crate::R<bool, TFC_PAUSE_A>;
impl TFC_PAUSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFC_PAUSE_A {
        match self.bits {
            false => TFC_PAUSE_A::_0,
            true => TFC_PAUSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFC_PAUSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFC_PAUSE_A::_1
    }
}
#[doc = "Write proxy for field `TFC_PAUSE`"]
pub struct TFC_PAUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFC_PAUSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFC_PAUSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No PAUSE frame transmitted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFC_PAUSE_A::_0)
    }
    #[doc = "The MAC stops transmission of data frames after the current transmission is complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFC_PAUSE_A::_1)
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
#[doc = "Reader of field `RFC_PAUSE`"]
pub type RFC_PAUSE_R = crate::R<bool, bool>;
#[doc = "Source MAC Address Select On Transmit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDSEL_A {
    #[doc = "0: Node MAC address programmed on PADDR1/2 registers."]
    _000,
}
impl From<ADDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDSEL_A) -> Self {
        match variant {
            ADDSEL_A::_000 => 0,
        }
    }
}
#[doc = "Reader of field `ADDSEL`"]
pub type ADDSEL_R = crate::R<u8, ADDSEL_A>;
impl ADDSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADDSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADDSEL_A::_000),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ADDSEL_A::_000
    }
}
#[doc = "Write proxy for field `ADDSEL`"]
pub struct ADDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Node MAC address programmed on PADDR1/2 registers."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ADDSEL_A::_000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Set MAC Address On Transmit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDINS_A {
    #[doc = "0: The source MAC address is not modified by the MAC."]
    _0,
    #[doc = "1: The MAC overwrites the source MAC address with the programmed MAC address according to ADDSEL."]
    _1,
}
impl From<ADDINS_A> for bool {
    #[inline(always)]
    fn from(variant: ADDINS_A) -> Self {
        match variant {
            ADDINS_A::_0 => false,
            ADDINS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ADDINS`"]
pub type ADDINS_R = crate::R<bool, ADDINS_A>;
impl ADDINS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDINS_A {
        match self.bits {
            false => ADDINS_A::_0,
            true => ADDINS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADDINS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADDINS_A::_1
    }
}
#[doc = "Write proxy for field `ADDINS`"]
pub struct ADDINS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDINS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDINS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The source MAC address is not modified by the MAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADDINS_A::_0)
    }
    #[doc = "The MAC overwrites the source MAC address with the programmed MAC address according to ADDSEL."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADDINS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Forward Frame From Application With CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCFWD_A {
    #[doc = "0: TxBD\\[TC\\] controls whether the frame has a CRC from the application."]
    _0,
    #[doc = "1: The transmitter does not append any CRC to transmitted frames, as it is expecting a frame with CRC from the application."]
    _1,
}
impl From<CRCFWD_A> for bool {
    #[inline(always)]
    fn from(variant: CRCFWD_A) -> Self {
        match variant {
            CRCFWD_A::_0 => false,
            CRCFWD_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `CRCFWD`"]
pub type CRCFWD_R = crate::R<bool, CRCFWD_A>;
impl CRCFWD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCFWD_A {
        match self.bits {
            false => CRCFWD_A::_0,
            true => CRCFWD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCFWD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCFWD_A::_1
    }
}
#[doc = "Write proxy for field `CRCFWD`"]
pub struct CRCFWD_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCFWD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCFWD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TxBD\\[TC\\] controls whether the frame has a CRC from the application."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCFWD_A::_0)
    }
    #[doc = "The transmitter does not append any CRC to transmitted frames, as it is expecting a frame with CRC from the application."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCFWD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Graceful Transmit Stop"]
    #[inline(always)]
    pub fn gts(&self) -> GTS_R {
        GTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Full-Duplex Enable"]
    #[inline(always)]
    pub fn fden(&self) -> FDEN_R {
        FDEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Frame Control Pause"]
    #[inline(always)]
    pub fn tfc_pause(&self) -> TFC_PAUSE_R {
        TFC_PAUSE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive Frame Control Pause"]
    #[inline(always)]
    pub fn rfc_pause(&self) -> RFC_PAUSE_R {
        RFC_PAUSE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Source MAC Address Select On Transmit"]
    #[inline(always)]
    pub fn addsel(&self) -> ADDSEL_R {
        ADDSEL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Set MAC Address On Transmit"]
    #[inline(always)]
    pub fn addins(&self) -> ADDINS_R {
        ADDINS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Forward Frame From Application With CRC"]
    #[inline(always)]
    pub fn crcfwd(&self) -> CRCFWD_R {
        CRCFWD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Graceful Transmit Stop"]
    #[inline(always)]
    pub fn gts(&mut self) -> GTS_W {
        GTS_W { w: self }
    }
    #[doc = "Bit 2 - Full-Duplex Enable"]
    #[inline(always)]
    pub fn fden(&mut self) -> FDEN_W {
        FDEN_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Frame Control Pause"]
    #[inline(always)]
    pub fn tfc_pause(&mut self) -> TFC_PAUSE_W {
        TFC_PAUSE_W { w: self }
    }
    #[doc = "Bits 5:7 - Source MAC Address Select On Transmit"]
    #[inline(always)]
    pub fn addsel(&mut self) -> ADDSEL_W {
        ADDSEL_W { w: self }
    }
    #[doc = "Bit 8 - Set MAC Address On Transmit"]
    #[inline(always)]
    pub fn addins(&mut self) -> ADDINS_W {
        ADDINS_W { w: self }
    }
    #[doc = "Bit 9 - Forward Frame From Application With CRC"]
    #[inline(always)]
    pub fn crcfwd(&mut self) -> CRCFWD_W {
        CRCFWD_W { w: self }
    }
}
