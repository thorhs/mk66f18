#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::ENDPT {
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
pub struct EPHSHKR {
    bits: bool,
}
impl EPHSHKR {
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
pub struct EPSTALLR {
    bits: bool,
}
impl EPSTALLR {
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
pub struct EPTXENR {
    bits: bool,
}
impl EPTXENR {
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
pub struct EPRXENR {
    bits: bool,
}
impl EPRXENR {
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
pub struct EPCTLDISR {
    bits: bool,
}
impl EPCTLDISR {
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
pub struct RETRYDISR {
    bits: bool,
}
impl RETRYDISR {
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
#[doc = "Possible values of the field `HOSTWOHUB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOSTWOHUBR {
    #[doc = "Low-speed device connected to Host through a hub. PRE_PID will be generated as required."]
    _0,
    #[doc = "Low-speed device directly connected. No hub, or no low-speed device attached."]
    _1,
}
impl HOSTWOHUBR {
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
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            HOSTWOHUBR::_0 => false,
            HOSTWOHUBR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HOSTWOHUBR {
        match value {
            false => HOSTWOHUBR::_0,
            true => HOSTWOHUBR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HOSTWOHUBR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HOSTWOHUBR::_1
    }
}
#[doc = r" Proxy"]
pub struct _EPHSHKW<'a> {
    w: &'a mut W,
}
impl<'a> _EPHSHKW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _EPSTALLW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPTXENW<'a> {
    w: &'a mut W,
}
impl<'a> _EPTXENW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPRXENW<'a> {
    w: &'a mut W,
}
impl<'a> _EPRXENW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _EPCTLDISW<'a> {
    w: &'a mut W,
}
impl<'a> _EPCTLDISW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RETRYDISW<'a> {
    w: &'a mut W,
}
impl<'a> _RETRYDISW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HOSTWOHUB`"]
pub enum HOSTWOHUBW {
    #[doc = "Low-speed device connected to Host through a hub. PRE_PID will be generated as required."]
    _0,
    #[doc = "Low-speed device directly connected. No hub, or no low-speed device attached."]
    _1,
}
impl HOSTWOHUBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HOSTWOHUBW::_0 => false,
            HOSTWOHUBW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HOSTWOHUBW<'a> {
    w: &'a mut W,
}
impl<'a> _HOSTWOHUBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HOSTWOHUBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low-speed device connected to Host through a hub. PRE_PID will be generated as required."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HOSTWOHUBW::_0)
    }
    #[doc = "Low-speed device directly connected. No hub, or no low-speed device attached."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HOSTWOHUBW::_1)
    }
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
    #[inline]
    pub fn ephshk(&self) -> EPHSHKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        EPHSHKR { bits }
    }
    #[doc = "Bit 1 - When set this bit indicates that the endpoint is called"]
    #[inline]
    pub fn epstall(&self) -> EPSTALLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        EPSTALLR { bits }
    }
    #[doc = "Bit 2 - This bit, when set, enables the endpoint for TX transfers. See"]
    #[inline]
    pub fn eptxen(&self) -> EPTXENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        EPTXENR { bits }
    }
    #[doc = "Bit 3 - This bit, when set, enables the endpoint for RX transfers. See"]
    #[inline]
    pub fn eprxen(&self) -> EPRXENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        EPRXENR { bits }
    }
    #[doc = "Bit 4 - This bit, when set, disables control (SETUP) transfers"]
    #[inline]
    pub fn epctldis(&self) -> EPCTLDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        EPCTLDISR { bits }
    }
    #[doc = "Bit 6 - This is a Host mode only bit and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline]
    pub fn retrydis(&self) -> RETRYDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        RETRYDISR { bits }
    }
    #[doc = "Bit 7 - Host without a hub This is a Host mode only field and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline]
    pub fn hostwohub(&self) -> HOSTWOHUBR {
        HOSTWOHUBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
    #[inline]
    pub fn ephshk(&mut self) -> _EPHSHKW {
        _EPHSHKW { w: self }
    }
    #[doc = "Bit 1 - When set this bit indicates that the endpoint is called"]
    #[inline]
    pub fn epstall(&mut self) -> _EPSTALLW {
        _EPSTALLW { w: self }
    }
    #[doc = "Bit 2 - This bit, when set, enables the endpoint for TX transfers. See"]
    #[inline]
    pub fn eptxen(&mut self) -> _EPTXENW {
        _EPTXENW { w: self }
    }
    #[doc = "Bit 3 - This bit, when set, enables the endpoint for RX transfers. See"]
    #[inline]
    pub fn eprxen(&mut self) -> _EPRXENW {
        _EPRXENW { w: self }
    }
    #[doc = "Bit 4 - This bit, when set, disables control (SETUP) transfers"]
    #[inline]
    pub fn epctldis(&mut self) -> _EPCTLDISW {
        _EPCTLDISW { w: self }
    }
    #[doc = "Bit 6 - This is a Host mode only bit and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline]
    pub fn retrydis(&mut self) -> _RETRYDISW {
        _RETRYDISW { w: self }
    }
    #[doc = "Bit 7 - Host without a hub This is a Host mode only field and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline]
    pub fn hostwohub(&mut self) -> _HOSTWOHUBW {
        _HOSTWOHUBW { w: self }
    }
}
