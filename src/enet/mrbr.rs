#[doc = "Reader of register MRBR"]
pub type R = crate::R<u32, super::MRBR>;
#[doc = "Writer for register MRBR"]
pub type W = crate::W<u32, super::MRBR>;
#[doc = "Register MRBR `reset()`'s with value 0"]
impl crate::ResetValue for super::MRBR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `R_BUF_SIZE`"]
pub type R_BUF_SIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `R_BUF_SIZE`"]
pub struct R_BUF_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> R_BUF_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 4)) | (((value as u32) & 0x7f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:10 - Receive buffer size in bytes"]
    #[inline(always)]
    pub fn r_buf_size(&self) -> R_BUF_SIZE_R {
        R_BUF_SIZE_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 4:10 - Receive buffer size in bytes"]
    #[inline(always)]
    pub fn r_buf_size(&mut self) -> R_BUF_SIZE_W {
        R_BUF_SIZE_W { w: self }
    }
}
