#[doc = "Reader of register ATCVH"]
pub type R = crate::R<u8, super::ATCVH>;
#[doc = "Writer for register ATCVH"]
pub type W = crate::W<u8, super::ATCVH>;
#[doc = "Register ATCVH `reset()`'s with value 0"]
impl crate::ResetValue for super::ATCVH {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ATCVH`"]
pub type ATCVH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATCVH`"]
pub struct ATCVH_W<'a> {
    w: &'a mut W,
}
impl<'a> ATCVH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u8) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ATM Compare Value High"]
    #[inline(always)]
    pub fn atcvh(&self) -> ATCVH_R {
        ATCVH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATM Compare Value High"]
    #[inline(always)]
    pub fn atcvh(&mut self) -> ATCVH_W {
        ATCVH_W { w: self }
    }
}
