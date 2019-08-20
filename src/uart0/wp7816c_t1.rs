#[doc = "Reader of register WP7816C_T1"]
pub type R = crate::R<u8, super::WP7816C_T1>;
#[doc = "Writer for register WP7816C_T1"]
pub type W = crate::W<u8, super::WP7816C_T1>;
#[doc = "Register WP7816C_T1 `reset()`'s with value 0x0b"]
impl crate::ResetValue for super::WP7816C_T1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0b
    }
}
#[doc = "Reader of field `CWI2`"]
pub type CWI2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CWI2`"]
pub struct CWI2_W<'a> {
    w: &'a mut W,
}
impl<'a> CWI2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u8) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Character Wait Time Integer 2 (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn cwi2(&self) -> CWI2_R {
        CWI2_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Character Wait Time Integer 2 (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn cwi2(&mut self) -> CWI2_W {
        CWI2_W { w: self }
    }
}
