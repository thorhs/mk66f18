#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TXFR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct TXDATAR {
    bits: u16,
}
impl TXDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXCMD_TXDATAR {
    bits: u16,
}
impl TXCMD_TXDATAR {
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
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline]
    pub fn txdata(&self) -> TXDATAR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TXDATAR { bits }
    }
    #[doc = "Bits 16:31 - Transmit Command or Transmit Data"]
    #[inline]
    pub fn txcmd_txdata(&self) -> TXCMD_TXDATAR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TXCMD_TXDATAR { bits }
    }
}
