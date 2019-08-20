#[doc = "Reader of register C2"]
pub type R = crate::R<u8, super::C2>;
#[doc = "Writer for register C2"]
pub type W = crate::W<u8, super::C2>;
#[doc = "Register C2 `reset()`'s with value 0"]
impl crate::ResetValue for super::C2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Send Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBK_A {
    #[doc = "0: Normal transmitter operation."]
    _0,
    #[doc = "1: Queue break characters to be sent."]
    _1,
}
impl From<SBK_A> for bool {
    #[inline(always)]
    fn from(variant: SBK_A) -> Self {
        match variant {
            SBK_A::_0 => false,
            SBK_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `SBK`"]
pub type SBK_R = crate::R<bool, SBK_A>;
impl SBK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBK_A {
        match self.bits {
            false => SBK_A::_0,
            true => SBK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBK_A::_1
    }
}
#[doc = "Write proxy for field `SBK`"]
pub struct SBK_W<'a> {
    w: &'a mut W,
}
impl<'a> SBK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal transmitter operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBK_A::_0)
    }
    #[doc = "Queue break characters to be sent."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBK_A::_1)
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
#[doc = "Receiver Wakeup Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWU_A {
    #[doc = "0: Normal operation."]
    _0,
    #[doc = "1: RWU enables the wakeup function and inhibits further receiver interrupt requests. Normally, hardware wakes the receiver by automatically clearing RWU."]
    _1,
}
impl From<RWU_A> for bool {
    #[inline(always)]
    fn from(variant: RWU_A) -> Self {
        match variant {
            RWU_A::_0 => false,
            RWU_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RWU`"]
pub type RWU_R = crate::R<bool, RWU_A>;
impl RWU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWU_A {
        match self.bits {
            false => RWU_A::_0,
            true => RWU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWU_A::_1
    }
}
#[doc = "Write proxy for field `RWU`"]
pub struct RWU_W<'a> {
    w: &'a mut W,
}
impl<'a> RWU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RWU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWU_A::_0)
    }
    #[doc = "RWU enables the wakeup function and inhibits further receiver interrupt requests. Normally, hardware wakes the receiver by automatically clearing RWU."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWU_A::_1)
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
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RE_A {
    #[doc = "0: Receiver off."]
    _0,
    #[doc = "1: Receiver on."]
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
    #[doc = "Receiver off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RE_A::_0)
    }
    #[doc = "Receiver on."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Transmitter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TE_A {
    #[doc = "0: Transmitter off."]
    _0,
    #[doc = "1: Transmitter on."]
    _1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        match variant {
            TE_A::_0 => false,
            TE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TE`"]
pub type TE_R = crate::R<bool, TE_A>;
impl TE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::_0,
            true => TE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TE_A::_1
    }
}
#[doc = "Write proxy for field `TE`"]
pub struct TE_W<'a> {
    w: &'a mut W,
}
impl<'a> TE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmitter off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TE_A::_0)
    }
    #[doc = "Transmitter on."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TE_A::_1)
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
#[doc = "Idle Line Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIE_A {
    #[doc = "0: IDLE interrupt requests disabled."]
    _0,
    #[doc = "1: IDLE interrupt requests enabled."]
    _1,
}
impl From<ILIE_A> for bool {
    #[inline(always)]
    fn from(variant: ILIE_A) -> Self {
        match variant {
            ILIE_A::_0 => false,
            ILIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ILIE`"]
pub type ILIE_R = crate::R<bool, ILIE_A>;
impl ILIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIE_A {
        match self.bits {
            false => ILIE_A::_0,
            true => ILIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILIE_A::_1
    }
}
#[doc = "Write proxy for field `ILIE`"]
pub struct ILIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ILIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IDLE interrupt requests disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILIE_A::_0)
    }
    #[doc = "IDLE interrupt requests enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILIE_A::_1)
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
#[doc = "Receiver Full Interrupt or DMA Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIE_A {
    #[doc = "0: RDRF interrupt and DMA transfer requests disabled."]
    _0,
    #[doc = "1: RDRF interrupt or DMA transfer requests enabled."]
    _1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        match variant {
            RIE_A::_0 => false,
            RIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RIE`"]
pub type RIE_R = crate::R<bool, RIE_A>;
impl RIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
#[doc = "Write proxy for field `RIE`"]
pub struct RIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RDRF interrupt and DMA transfer requests disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIE_A::_0)
    }
    #[doc = "RDRF interrupt or DMA transfer requests enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIE_A::_1)
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
#[doc = "Transmission Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIE_A {
    #[doc = "0: TC interrupt requests disabled."]
    _0,
    #[doc = "1: TC interrupt requests enabled."]
    _1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        match variant {
            TCIE_A::_0 => false,
            TCIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TCIE`"]
pub type TCIE_R = crate::R<bool, TCIE_A>;
impl TCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::_0,
            true => TCIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCIE_A::_1
    }
}
#[doc = "Write proxy for field `TCIE`"]
pub struct TCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TC interrupt requests disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCIE_A::_0)
    }
    #[doc = "TC interrupt requests enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCIE_A::_1)
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
#[doc = "Transmitter Interrupt or DMA Transfer Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIE_A {
    #[doc = "0: TDRE interrupt and DMA transfer requests disabled."]
    _0,
    #[doc = "1: TDRE interrupt or DMA transfer requests enabled."]
    _1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        match variant {
            TIE_A::_0 => false,
            TIE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TIE`"]
pub type TIE_R = crate::R<bool, TIE_A>;
impl TIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
#[doc = "Write proxy for field `TIE`"]
pub struct TIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TDRE interrupt and DMA transfer requests disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "TDRE interrupt or DMA transfer requests enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
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
    #[doc = "Bit 0 - Send Break"]
    #[inline(always)]
    pub fn sbk(&self) -> SBK_R {
        SBK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receiver Wakeup Control"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Idle Line Interrupt Enable"]
    #[inline(always)]
    pub fn ilie(&self) -> ILIE_R {
        ILIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receiver Full Interrupt or DMA Transfer Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmission Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmitter Interrupt or DMA Transfer Enable."]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send Break"]
    #[inline(always)]
    pub fn sbk(&mut self) -> SBK_W {
        SBK_W { w: self }
    }
    #[doc = "Bit 1 - Receiver Wakeup Control"]
    #[inline(always)]
    pub fn rwu(&mut self) -> RWU_W {
        RWU_W { w: self }
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> RE_W {
        RE_W { w: self }
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&mut self) -> TE_W {
        TE_W { w: self }
    }
    #[doc = "Bit 4 - Idle Line Interrupt Enable"]
    #[inline(always)]
    pub fn ilie(&mut self) -> ILIE_W {
        ILIE_W { w: self }
    }
    #[doc = "Bit 5 - Receiver Full Interrupt or DMA Transfer Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W {
        RIE_W { w: self }
    }
    #[doc = "Bit 6 - Transmission Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W {
        TCIE_W { w: self }
    }
    #[doc = "Bit 7 - Transmitter Interrupt or DMA Transfer Enable."]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W {
        TIE_W { w: self }
    }
}
