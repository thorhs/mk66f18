#[doc = "Reader of register TSHD"]
pub type R = crate::R<u32, super::TSHD>;
#[doc = "Writer for register TSHD"]
pub type W = crate::W<u32, super::TSHD>;
#[doc = "Register TSHD `reset()`'s with value 0"]
impl crate::ResetValue for super::TSHD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `THRESL`"]
pub type THRESL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `THRESL`"]
pub struct THRESL_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `THRESH`"]
pub type THRESH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `THRESH`"]
pub struct THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TSI Wakeup Channel Low-threshold"]
    #[inline(always)]
    pub fn thresl(&self) -> THRESL_R {
        THRESL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TSI Wakeup Channel High-threshold"]
    #[inline(always)]
    pub fn thresh(&self) -> THRESH_R {
        THRESH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TSI Wakeup Channel Low-threshold"]
    #[inline(always)]
    pub fn thresl(&mut self) -> THRESL_W {
        THRESL_W { w: self }
    }
    #[doc = "Bits 16:31 - TSI Wakeup Channel High-threshold"]
    #[inline(always)]
    pub fn thresh(&mut self) -> THRESH_W {
        THRESH_W { w: self }
    }
}
