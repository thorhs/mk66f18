#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USB1_LOOPBACK_HSFSCNT_TOG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct TSTI_HS_NUMBERR {
    bits: u16,
}
impl TSTI_HS_NUMBERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TSTI_FS_NUMBERR {
    bits: u16,
}
impl TSTI_FS_NUMBERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TSTI_HS_NUMBERW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTI_HS_NUMBERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSTI_FS_NUMBERW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTI_FS_NUMBERW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - High speed packet number, used when USBPHY_USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline]
    pub fn tsti_hs_number(&self) -> TSTI_HS_NUMBERR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TSTI_HS_NUMBERR { bits }
    }
    #[doc = "Bits 16:31 - Full speed packet number, used when USBPHY_USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline]
    pub fn tsti_fs_number(&self) -> TSTI_FS_NUMBERR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TSTI_FS_NUMBERR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 262160 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - High speed packet number, used when USBPHY_USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline]
    pub fn tsti_hs_number(&mut self) -> _TSTI_HS_NUMBERW {
        _TSTI_HS_NUMBERW { w: self }
    }
    #[doc = "Bits 16:31 - Full speed packet number, used when USBPHY_USB1_LOOPBACK\\[TSTI_HSFS_MODE_EN\\] is set to value 1'b1."]
    #[inline]
    pub fn tsti_fs_number(&mut self) -> _TSTI_FS_NUMBERW {
        _TSTI_FS_NUMBERW { w: self }
    }
}
