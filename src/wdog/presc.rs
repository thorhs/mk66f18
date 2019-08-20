#[doc = "Reader of register PRESC"]
pub type R = crate::R<u16, super::PRESC>;
#[doc = "Writer for register PRESC"]
pub type W = crate::W<u16, super::PRESC>;
#[doc = "Register PRESC `reset()`'s with value 0x0400"]
impl crate::ResetValue for super::PRESC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400
    }
}
#[doc = "Reader of field `PRESCVAL`"]
pub type PRESCVAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRESCVAL`"]
pub struct PRESCVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u16) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:10 - 3-bit prescaler for the watchdog clock source"]
    #[inline(always)]
    pub fn prescval(&self) -> PRESCVAL_R {
        PRESCVAL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 8:10 - 3-bit prescaler for the watchdog clock source"]
    #[inline(always)]
    pub fn prescval(&mut self) -> PRESCVAL_W {
        PRESCVAL_W { w: self }
    }
}
