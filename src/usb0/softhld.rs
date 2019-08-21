#[doc = "Reader of register SOFTHLD"]
pub type R = crate::R<u8, super::SOFTHLD>;
#[doc = "Writer for register SOFTHLD"]
pub type W = crate::W<u8, super::SOFTHLD>;
#[doc = "Register SOFTHLD `reset()`'s with value 0"]
impl crate::ResetValue for super::SOFTHLD {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CNT`"]
pub type CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CNT`"]
pub struct CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Represents the SOF count threshold in byte times."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Represents the SOF count threshold in byte times."]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W {
        CNT_W { w: self }
    }
}
