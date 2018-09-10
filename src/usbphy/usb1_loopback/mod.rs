#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USB1_LOOPBACK {
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
pub struct UTMI_TESTSTARTR {
    bits: bool,
}
impl UTMI_TESTSTARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct UTMI_DIG_TST0R {
    bits: bool,
}
impl UTMI_DIG_TST0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct UTMI_DIG_TST1R {
    bits: bool,
}
impl UTMI_DIG_TST1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TSTI_TX_HS_MODER {
    bits: bool,
}
impl TSTI_TX_HS_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TSTI_TX_LS_MODER {
    bits: bool,
}
impl TSTI_TX_LS_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TSTI_TX_ENR {
    bits: bool,
}
impl TSTI_TX_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TSTI_TX_HIZR {
    bits: bool,
}
impl TSTI_TX_HIZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct UTMO_DIG_TST0R {
    bits: bool,
}
impl UTMO_DIG_TST0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct UTMO_DIG_TST1R {
    bits: bool,
}
impl UTMO_DIG_TST1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TSTI_HSFS_MODE_ENR {
    bits: bool,
}
impl TSTI_HSFS_MODE_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct TSTPKTR {
    bits: u8,
}
impl TSTPKTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _UTMI_TESTSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _UTMI_TESTSTARTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UTMI_DIG_TST0W<'a> {
    w: &'a mut W,
}
impl<'a> _UTMI_DIG_TST0W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UTMI_DIG_TST1W<'a> {
    w: &'a mut W,
}
impl<'a> _UTMI_DIG_TST1W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSTI_TX_HS_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTI_TX_HS_MODEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSTI_TX_LS_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTI_TX_LS_MODEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSTI_TX_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTI_TX_ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSTI_TX_HIZW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTI_TX_HIZW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSTI_HSFS_MODE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTI_HSFS_MODE_ENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSTPKTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTPKTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 0 - This bit enables the USB loopback test."]
    #[inline]
    pub fn utmi_teststart(&self) -> UTMI_TESTSTARTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UTMI_TESTSTARTR { bits }
    }
    #[doc = "Bit 1 - Mode control for USB loopback test"]
    #[inline]
    pub fn utmi_dig_tst0(&self) -> UTMI_DIG_TST0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UTMI_DIG_TST0R { bits }
    }
    #[doc = "Bit 2 - Mode control for USB loopback test"]
    #[inline]
    pub fn utmi_dig_tst1(&self) -> UTMI_DIG_TST1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UTMI_DIG_TST1R { bits }
    }
    #[doc = "Bit 3 - Select HS or FS mode for USB loopback testing"]
    #[inline]
    pub fn tsti_tx_hs_mode(&self) -> TSTI_TX_HS_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSTI_TX_HS_MODER { bits }
    }
    #[doc = "Bit 4 - Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[inline]
    pub fn tsti_tx_ls_mode(&self) -> TSTI_TX_LS_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSTI_TX_LS_MODER { bits }
    }
    #[doc = "Bit 5 - Enable TX for USB loopback test."]
    #[inline]
    pub fn tsti_tx_en(&self) -> TSTI_TX_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSTI_TX_ENR { bits }
    }
    #[doc = "Bit 6 - Sets TX Hi-Z for USB loopback test."]
    #[inline]
    pub fn tsti_tx_hiz(&self) -> TSTI_TX_HIZR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSTI_TX_HIZR { bits }
    }
    #[doc = "Bit 7 - This read-only bit is a status bit for USB loopback test results"]
    #[inline]
    pub fn utmo_dig_tst0(&self) -> UTMO_DIG_TST0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UTMO_DIG_TST0R { bits }
    }
    #[doc = "Bit 8 - This read-only bit is a status bit for USB loopback test"]
    #[inline]
    pub fn utmo_dig_tst1(&self) -> UTMO_DIG_TST1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UTMO_DIG_TST1R { bits }
    }
    #[doc = "Bit 15 - Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[inline]
    pub fn tsti_hsfs_mode_en(&self) -> TSTI_HSFS_MODE_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSTI_HSFS_MODE_ENR { bits }
    }
    #[doc = "Bits 16:23 - Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[inline]
    pub fn tstpkt(&self) -> TSTPKTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TSTPKTR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 5570560 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - This bit enables the USB loopback test."]
    #[inline]
    pub fn utmi_teststart(&mut self) -> _UTMI_TESTSTARTW {
        _UTMI_TESTSTARTW { w: self }
    }
    #[doc = "Bit 1 - Mode control for USB loopback test"]
    #[inline]
    pub fn utmi_dig_tst0(&mut self) -> _UTMI_DIG_TST0W {
        _UTMI_DIG_TST0W { w: self }
    }
    #[doc = "Bit 2 - Mode control for USB loopback test"]
    #[inline]
    pub fn utmi_dig_tst1(&mut self) -> _UTMI_DIG_TST1W {
        _UTMI_DIG_TST1W { w: self }
    }
    #[doc = "Bit 3 - Select HS or FS mode for USB loopback testing"]
    #[inline]
    pub fn tsti_tx_hs_mode(&mut self) -> _TSTI_TX_HS_MODEW {
        _TSTI_TX_HS_MODEW { w: self }
    }
    #[doc = "Bit 4 - Set to value 1'b1 to choose LS for USB loopback testing, set to value 1'b0 to choose HS or FS mode which is defined by TSTI1_TX_HS"]
    #[inline]
    pub fn tsti_tx_ls_mode(&mut self) -> _TSTI_TX_LS_MODEW {
        _TSTI_TX_LS_MODEW { w: self }
    }
    #[doc = "Bit 5 - Enable TX for USB loopback test."]
    #[inline]
    pub fn tsti_tx_en(&mut self) -> _TSTI_TX_ENW {
        _TSTI_TX_ENW { w: self }
    }
    #[doc = "Bit 6 - Sets TX Hi-Z for USB loopback test."]
    #[inline]
    pub fn tsti_tx_hiz(&mut self) -> _TSTI_TX_HIZW {
        _TSTI_TX_HIZW { w: self }
    }
    #[doc = "Bit 15 - Setting this bit field to value 1'b1 will enable the loopback test to dynamically change the packet speed"]
    #[inline]
    pub fn tsti_hsfs_mode_en(&mut self) -> _TSTI_HSFS_MODE_ENW {
        _TSTI_HSFS_MODE_ENW { w: self }
    }
    #[doc = "Bits 16:23 - Selects the packet data byte used for USB loopback testing in Pulse mode"]
    #[inline]
    pub fn tstpkt(&mut self) -> _TSTPKTW {
        _TSTPKTW { w: self }
    }
}
