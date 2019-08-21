#[doc = "Reader of register WP7816A_T1"]
pub type R = crate::R<u8, super::WP7816A_T1>;
#[doc = "Writer for register WP7816A_T1"]
pub type W = crate::W<u8, super::WP7816A_T1>;
#[doc = "Register WP7816A_T1 `reset()`'s with value 0"]
impl crate::ResetValue for super::WP7816A_T1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BWI_H`"]
pub type BWI_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BWI_H`"]
pub struct BWI_H_W<'a> {
    w: &'a mut W,
}
impl<'a> BWI_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Block Wait Time Integer High (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn bwi_h(&self) -> BWI_H_R {
        BWI_H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Block Wait Time Integer High (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn bwi_h(&mut self) -> BWI_H_W {
        BWI_H_W { w: self }
    }
}
