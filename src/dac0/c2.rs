#[doc = "Reader of register C2"]
pub type R = crate::R<u8, super::C2>;
#[doc = "Writer for register C2"]
pub type W = crate::W<u8, super::C2>;
#[doc = "Register C2 `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::C2 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `DACBFUP`"]
pub type DACBFUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DACBFUP`"]
pub struct DACBFUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DACBFRP`"]
pub type DACBFRP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DACBFRP`"]
pub struct DACBFRP_W<'a> {
    w: &'a mut W,
}
impl<'a> DACBFRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u8) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DAC Buffer Upper Limit"]
    #[inline(always)]
    pub fn dacbfup(&self) -> DACBFUP_R {
        DACBFUP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DAC Buffer Read Pointer"]
    #[inline(always)]
    pub fn dacbfrp(&self) -> DACBFRP_R {
        DACBFRP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DAC Buffer Upper Limit"]
    #[inline(always)]
    pub fn dacbfup(&mut self) -> DACBFUP_W {
        DACBFUP_W { w: self }
    }
    #[doc = "Bits 4:7 - DAC Buffer Read Pointer"]
    #[inline(always)]
    pub fn dacbfrp(&mut self) -> DACBFRP_W {
        DACBFRP_W { w: self }
    }
}
