#[doc = "Reader of register TAGVDW0S%s"]
pub type R = crate::R<u32, super::TAGVDW0S>;
#[doc = "Writer for register TAGVDW0S%s"]
pub type W = crate::W<u32, super::TAGVDW0S>;
#[doc = "Register TAGVDW0S%s `reset()`'s with value 0"]
impl crate::ResetValue for super::TAGVDW0S {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `valid`"]
pub type VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `valid`"]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `tag`"]
pub type TAG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `tag`"]
pub struct TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 6)) | (((value as u32) & 0xffff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1-bit valid for cache entry"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 6:21 - 16-bit tag for cache entry"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 1-bit valid for cache entry"]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
    }
    #[doc = "Bits 6:21 - 16-bit tag for cache entry"]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W {
        TAG_W { w: self }
    }
}
