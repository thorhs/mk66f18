#[doc = "Reader of register UNLOCK"]
pub type R = crate::R<u16, super::UNLOCK>;
#[doc = "Writer for register UNLOCK"]
pub type W = crate::W<u16, super::UNLOCK>;
#[doc = "Register UNLOCK `reset()`'s with value 0xd928"]
impl crate::ResetValue for super::UNLOCK {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xd928
    }
}
#[doc = "Reader of field `WDOGUNLOCK`"]
pub type WDOGUNLOCK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WDOGUNLOCK`"]
pub struct WDOGUNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDOGUNLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Writing the unlock sequence values to this register to makes the watchdog write-once registers writable again"]
    #[inline(always)]
    pub fn wdogunlock(&self) -> WDOGUNLOCK_R {
        WDOGUNLOCK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Writing the unlock sequence values to this register to makes the watchdog write-once registers writable again"]
    #[inline(always)]
    pub fn wdogunlock(&mut self) -> WDOGUNLOCK_W {
        WDOGUNLOCK_W { w: self }
    }
}
