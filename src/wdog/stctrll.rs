#[doc = "Reader of register STCTRLL"]
pub type R = crate::R<u16, super::STCTRLL>;
#[doc = "Writer for register STCTRLL"]
pub type W = crate::W<u16, super::STCTRLL>;
#[doc = "Register STCTRLL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::STCTRLL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `INTFLG`"]
pub type INTFLG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INTFLG`"]
pub struct INTFLG_W<'a> {
    w: &'a mut W,
}
impl<'a> INTFLG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u16) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - Interrupt flag"]
    #[inline(always)]
    pub fn intflg(&self) -> INTFLG_R {
        INTFLG_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Interrupt flag"]
    #[inline(always)]
    pub fn intflg(&mut self) -> INTFLG_W {
        INTFLG_W { w: self }
    }
}
