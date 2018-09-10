#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::HCIVERSION {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CAPLENGTHR {
    bits: u8,
}
impl CAPLENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HCIVERSIONR {
    bits: u16,
}
impl HCIVERSIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Capability registers length"]
    #[inline]
    pub fn caplength(&self) -> CAPLENGTHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CAPLENGTHR { bits }
    }
    #[doc = "Bits 16:31 - EHCI revision number"]
    #[inline]
    pub fn hciversion(&self) -> HCIVERSIONR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HCIVERSIONR { bits }
    }
}
