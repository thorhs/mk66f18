#[doc = "Reader of register USB1_LOOPBACK_HSFSCNT"]
pub type R = crate::R<u32, super::USB1_LOOPBACK_HSFSCNT>;
#[doc = "Writer for register USB1_LOOPBACK_HSFSCNT"]
pub type W = crate::W<u32, super::USB1_LOOPBACK_HSFSCNT>;
#[doc = "Register USB1_LOOPBACK_HSFSCNT `reset()`'s with value 0x0004_0010"]
impl crate::ResetValue for super::USB1_LOOPBACK_HSFSCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_0010
    }
}
#[doc = "Reader of field `TSTI_HS_NUMBER`"]
pub type TSTI_HS_NUMBER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSTI_HS_NUMBER`"]
pub struct TSTI_HS_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTI_HS_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `TSTI_FS_NUMBER`"]
pub type TSTI_FS_NUMBER_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSTI_FS_NUMBER`"]
pub struct TSTI_FS_NUMBER_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTI_FS_NUMBER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - High speed packet number, used when USBPHY_USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline(always)]
    pub fn tsti_hs_number(&self) -> TSTI_HS_NUMBER_R {
        TSTI_HS_NUMBER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Full speed packet number, used when USBPHY_USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline(always)]
    pub fn tsti_fs_number(&self) -> TSTI_FS_NUMBER_R {
        TSTI_FS_NUMBER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - High speed packet number, used when USBPHY_USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline(always)]
    pub fn tsti_hs_number(&mut self) -> TSTI_HS_NUMBER_W {
        TSTI_HS_NUMBER_W { w: self }
    }
    #[doc = "Bits 16:31 - Full speed packet number, used when USBPHY_USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline(always)]
    pub fn tsti_fs_number(&mut self) -> TSTI_FS_NUMBER_W {
        TSTI_FS_NUMBER_W { w: self }
    }
}
