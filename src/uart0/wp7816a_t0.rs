#[doc = "Reader of register WP7816A_T0"]
pub type R = crate::R<u8, super::WP7816A_T0>;
#[doc = "Writer for register WP7816A_T0"]
pub type W = crate::W<u8, super::WP7816A_T0>;
#[doc = "Register WP7816A_T0 `reset()`'s with value 0"]
impl crate::ResetValue for super::WP7816A_T0 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WI_H`"]
pub type WI_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WI_H`"]
pub struct WI_H_W<'a> {
    w: &'a mut W,
}
impl<'a> WI_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Wait Time Integer High (C7816\\[TTYPE\\] = 0)"]
    #[inline(always)]
    pub fn wi_h(&self) -> WI_H_R {
        WI_H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait Time Integer High (C7816\\[TTYPE\\] = 0)"]
    #[inline(always)]
    pub fn wi_h(&mut self) -> WI_H_W {
        WI_H_W { w: self }
    }
}
