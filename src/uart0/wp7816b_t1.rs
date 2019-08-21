#[doc = "Reader of register WP7816B_T1"]
pub type R = crate::R<u8, super::WP7816B_T1>;
#[doc = "Writer for register WP7816B_T1"]
pub type W = crate::W<u8, super::WP7816B_T1>;
#[doc = "Register WP7816B_T1 `reset()`'s with value 0x14"]
impl crate::ResetValue for super::WP7816B_T1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x14
    }
}
#[doc = "Reader of field `BWI_L`"]
pub type BWI_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BWI_L`"]
pub struct BWI_L_W<'a> {
    w: &'a mut W,
}
impl<'a> BWI_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Block Wait Time Integer Low (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn bwi_l(&self) -> BWI_L_R {
        BWI_L_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Block Wait Time Integer Low (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn bwi_l(&mut self) -> BWI_L_W {
        BWI_L_W { w: self }
    }
}
