#[doc = "Reader of register REG%s"]
pub type R = crate::R<u32, super::REG>;
#[doc = "Writer for register REG%s"]
pub type W = crate::W<u32, super::REG>;
#[doc = "Register REG%s `reset()`'s with value 0"]
impl crate::ResetValue for super::REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LL`"]
pub type LL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LL`"]
pub struct LL_W<'a> {
    w: &'a mut W,
}
impl<'a> LL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `LH`"]
pub type LH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LH`"]
pub struct LH_W<'a> {
    w: &'a mut W,
}
impl<'a> LH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HL`"]
pub type HL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HL`"]
pub struct HL_W<'a> {
    w: &'a mut W,
}
impl<'a> HL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HH`"]
pub type HH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HH`"]
pub struct HH_W<'a> {
    w: &'a mut W,
}
impl<'a> HH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Low lower byte"]
    #[inline(always)]
    pub fn ll(&self) -> LL_R {
        LL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Low higher byte"]
    #[inline(always)]
    pub fn lh(&self) -> LH_R {
        LH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - High lower byte"]
    #[inline(always)]
    pub fn hl(&self) -> HL_R {
        HL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - High higher byte"]
    #[inline(always)]
    pub fn hh(&self) -> HH_R {
        HH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low lower byte"]
    #[inline(always)]
    pub fn ll(&mut self) -> LL_W {
        LL_W { w: self }
    }
    #[doc = "Bits 8:15 - Low higher byte"]
    #[inline(always)]
    pub fn lh(&mut self) -> LH_W {
        LH_W { w: self }
    }
    #[doc = "Bits 16:23 - High lower byte"]
    #[inline(always)]
    pub fn hl(&mut self) -> HL_W {
        HL_W { w: self }
    }
    #[doc = "Bits 24:31 - High higher byte"]
    #[inline(always)]
    pub fn hh(&mut self) -> HH_W {
        HH_W { w: self }
    }
}
