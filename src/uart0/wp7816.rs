#[doc = "Reader of register WP7816"]
pub type R = crate::R<u8, super::WP7816>;
#[doc = "Writer for register WP7816"]
pub type W = crate::W<u8, super::WP7816>;
#[doc = "Register WP7816 `reset()`'s with value 0"]
impl crate::ResetValue for super::WP7816 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTX`"]
pub type WTX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WTX`"]
pub struct WTX_W<'a> {
    w: &'a mut W,
}
impl<'a> WTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Wait Time Multiplier (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn wtx(&self) -> WTX_R {
        WTX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait Time Multiplier (C7816\\[TTYPE\\] = 1)"]
    #[inline(always)]
    pub fn wtx(&mut self) -> WTX_W {
        WTX_W { w: self }
    }
}
