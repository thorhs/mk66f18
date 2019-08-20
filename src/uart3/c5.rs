#[doc = "Reader of register C5"]
pub type R = crate::R<u8, super::C5>;
#[doc = "Writer for register C5"]
pub type W = crate::W<u8, super::C5>;
#[doc = "Register C5 `reset()`'s with value 0"]
impl crate::ResetValue for super::C5 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Receiver Full DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMAS_A {
    #[doc = "0: If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDFR interrupt request signal is asserted to request an interrupt service."]
    _0,
    #[doc = "1: If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDRF DMA request signal is asserted to request a DMA transfer."]
    _1,
}
impl From<RDMAS_A> for bool {
    #[inline(always)]
    fn from(variant: RDMAS_A) -> Self {
        match variant {
            RDMAS_A::_0 => false,
            RDMAS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RDMAS`"]
pub type RDMAS_R = crate::R<bool, RDMAS_A>;
impl RDMAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMAS_A {
        match self.bits {
            false => RDMAS_A::_0,
            true => RDMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDMAS_A::_1
    }
}
#[doc = "Write proxy for field `RDMAS`"]
pub struct RDMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDMAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDFR interrupt request signal is asserted to request an interrupt service."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDMAS_A::_0)
    }
    #[doc = "If C2\\[RIE\\] and S1\\[RDRF\\] are set, the RDRF DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDMAS_A::_1)
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
#[doc = "Transmitter DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDMAS_A {
    #[doc = "0: If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE interrupt request signal is asserted to request interrupt service."]
    _0,
    #[doc = "1: If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE DMA request signal is asserted to request a DMA transfer."]
    _1,
}
impl From<TDMAS_A> for bool {
    #[inline(always)]
    fn from(variant: TDMAS_A) -> Self {
        match variant {
            TDMAS_A::_0 => false,
            TDMAS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `TDMAS`"]
pub type TDMAS_R = crate::R<bool, TDMAS_A>;
impl TDMAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDMAS_A {
        match self.bits {
            false => TDMAS_A::_0,
            true => TDMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDMAS_A::_1
    }
}
#[doc = "Write proxy for field `TDMAS`"]
pub struct TDMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TDMAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDMAS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE interrupt request signal is asserted to request interrupt service."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDMAS_A::_0)
    }
    #[doc = "If C2\\[TIE\\] is set and the S1\\[TDRE\\] flag is set, the TDRE DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDMAS_A::_1)
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
    #[doc = "Bit 5 - Receiver Full DMA Select"]
    #[inline(always)]
    pub fn rdmas(&self) -> RDMAS_R {
        RDMAS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transmitter DMA Select"]
    #[inline(always)]
    pub fn tdmas(&self) -> TDMAS_R {
        TDMAS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Receiver Full DMA Select"]
    #[inline(always)]
    pub fn rdmas(&mut self) -> RDMAS_W {
        RDMAS_W { w: self }
    }
    #[doc = "Bit 7 - Transmitter DMA Select"]
    #[inline(always)]
    pub fn tdmas(&mut self) -> TDMAS_W {
        TDMAS_W { w: self }
    }
}
