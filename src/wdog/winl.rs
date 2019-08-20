#[doc = "Reader of register WINL"]
pub type R = crate::R<u16, super::WINL>;
#[doc = "Writer for register WINL"]
pub type W = crate::W<u16, super::WINL>;
#[doc = "Register WINL `reset()`'s with value 0x10"]
impl crate::ResetValue for super::WINL {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `WINLOW`"]
pub type WINLOW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WINLOW`"]
pub struct WINLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> WINLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Defines the lower 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
    #[inline(always)]
    pub fn winlow(&self) -> WINLOW_R {
        WINLOW_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the lower 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
    #[inline(always)]
    pub fn winlow(&mut self) -> WINLOW_W {
        WINLOW_W { w: self }
    }
}
