#[doc = "Reader of register WML"]
pub type R = crate::R<u32, super::WML>;
#[doc = "Writer for register WML"]
pub type W = crate::W<u32, super::WML>;
#[doc = "Register WML `reset()`'s with value 0x0010_0010"]
impl crate::ResetValue for super::WML {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_0010
    }
}
#[doc = "Reader of field `RDWML`"]
pub type RDWML_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDWML`"]
pub struct RDWML_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWML_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `WRWML`"]
pub type WRWML_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRWML`"]
pub struct WRWML_W<'a> {
    w: &'a mut W,
}
impl<'a> WRWML_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Read Watermark Level"]
    #[inline(always)]
    pub fn rdwml(&self) -> RDWML_R {
        RDWML_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write Watermark Level"]
    #[inline(always)]
    pub fn wrwml(&self) -> WRWML_R {
        WRWML_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Watermark Level"]
    #[inline(always)]
    pub fn rdwml(&mut self) -> RDWML_W {
        RDWML_W { w: self }
    }
    #[doc = "Bits 16:23 - Write Watermark Level"]
    #[inline(always)]
    pub fn wrwml(&mut self) -> WRWML_W {
        WRWML_W { w: self }
    }
}
