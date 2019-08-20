#[doc = "Reader of register USBFRMADJUST"]
pub type R = crate::R<u8, super::USBFRMADJUST>;
#[doc = "Writer for register USBFRMADJUST"]
pub type W = crate::W<u8, super::USBFRMADJUST>;
#[doc = "Register USBFRMADJUST `reset()`'s with value 0"]
impl crate::ResetValue for super::USBFRMADJUST {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADJ`"]
pub type ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADJ`"]
pub struct ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Frame Adjustment"]
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Adjustment"]
    #[inline(always)]
    pub fn adj(&mut self) -> ADJ_W {
        ADJ_W { w: self }
    }
}
