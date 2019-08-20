#[doc = "Reader of register ETBRL"]
pub type R = crate::R<u32, super::ETBRL>;
#[doc = "Writer for register ETBRL"]
pub type W = crate::W<u32, super::ETBRL>;
#[doc = "Register ETBRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ETBRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RELOAD`"]
pub type RELOAD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RELOAD`"]
pub struct RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Byte Count Reload Value"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Byte Count Reload Value"]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W {
        RELOAD_W { w: self }
    }
}
