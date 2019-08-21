#[doc = "Reader of register TCR"]
pub type R = crate::R<u32, super::TCR>;
#[doc = "Writer for register TCR"]
pub type W = crate::W<u32, super::TCR>;
#[doc = "Register TCR `reset()`'s with value 0"]
impl crate::ResetValue for super::TCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_TCNT`"]
pub type SPI_TCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPI_TCNT`"]
pub struct SPI_TCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_TCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - SPI Transfer Counter"]
    #[inline(always)]
    pub fn spi_tcnt(&self) -> SPI_TCNT_R {
        SPI_TCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - SPI Transfer Counter"]
    #[inline(always)]
    pub fn spi_tcnt(&mut self) -> SPI_TCNT_W {
        SPI_TCNT_W { w: self }
    }
}
