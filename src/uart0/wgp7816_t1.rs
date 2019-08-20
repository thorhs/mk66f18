#[doc = "Reader of register WGP7816_T1"]
pub type R = crate::R<u8, super::WGP7816_T1>;
#[doc = "Writer for register WGP7816_T1"]
pub type W = crate::W<u8, super::WGP7816_T1>;
#[doc = "Register WGP7816_T1 `reset()`'s with value 0x06"]
impl crate::ResetValue for super::WGP7816_T1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06
    }
}
#[doc = "Reader of field `BGI`"]
pub type BGI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BGI`"]
pub struct BGI_W<'a> {
    w: &'a mut W,
}
impl<'a> BGI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `CWI1`"]
pub type CWI1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CWI1`"]
pub struct CWI1_W<'a> {
    w: &'a mut W,
}
impl<'a> CWI1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Block Guard Time Integer (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn bgi(&self) -> BGI_R {
        BGI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Character Wait Time Integer 1 (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn cwi1(&self) -> CWI1_R {
        CWI1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Block Guard Time Integer (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn bgi(&mut self) -> BGI_W {
        BGI_W { w: self }
    }
    #[doc = "Bits 4:7 - Character Wait Time Integer 1 (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn cwi1(&mut self) -> CWI1_W {
        CWI1_W { w: self }
    }
}
