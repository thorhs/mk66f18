#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::SSRS1 {
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
#[doc = "Possible values of the field `SJTAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SJTAGR {
    #[doc = "Reset not caused by JTAG"]
    _0,
    #[doc = "Reset caused by JTAG"]
    _1,
}
impl SJTAGR {
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
            SJTAGR::_0 => false,
            SJTAGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SJTAGR {
        match value {
            false => SJTAGR::_0,
            true => SJTAGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SJTAGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SJTAGR::_1
    }
}
#[doc = "Possible values of the field `SLOCKUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOCKUPR {
    #[doc = "Reset not caused by core LOCKUP event"]
    _0,
    #[doc = "Reset caused by core LOCKUP event"]
    _1,
}
impl SLOCKUPR {
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
            SLOCKUPR::_0 => false,
            SLOCKUPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLOCKUPR {
        match value {
            false => SLOCKUPR::_0,
            true => SLOCKUPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLOCKUPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLOCKUPR::_1
    }
}
#[doc = "Possible values of the field `SSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSWR {
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    _0,
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    _1,
}
impl SSWR {
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
            SSWR::_0 => false,
            SSWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSWR {
        match value {
            false => SSWR::_0,
            true => SSWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SSWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SSWR::_1
    }
}
#[doc = "Possible values of the field `SMDM_AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMDM_APR {
    #[doc = "Reset not caused by host debugger system setting of the System Reset Request bit"]
    _0,
    #[doc = "Reset caused by host debugger system setting of the System Reset Request bit"]
    _1,
}
impl SMDM_APR {
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
            SMDM_APR::_0 => false,
            SMDM_APR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMDM_APR {
        match value {
            false => SMDM_APR::_0,
            true => SMDM_APR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SMDM_APR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SMDM_APR::_1
    }
}
#[doc = "Possible values of the field `SEZPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEZPTR {
    #[doc = "Reset not caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    _0,
    #[doc = "Reset caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    _1,
}
impl SEZPTR {
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
            SEZPTR::_0 => false,
            SEZPTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEZPTR {
        match value {
            false => SEZPTR::_0,
            true => SEZPTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SEZPTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SEZPTR::_1
    }
}
#[doc = "Possible values of the field `SSACKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSACKERRR {
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _0,
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _1,
}
impl SSACKERRR {
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
            SSACKERRR::_0 => false,
            SSACKERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSACKERRR {
        match value {
            false => SSACKERRR::_0,
            true => SSACKERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SSACKERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SSACKERRR::_1
    }
}
#[doc = "Values that can be written to the field `SJTAG`"]
pub enum SJTAGW {
    #[doc = "Reset not caused by JTAG"]
    _0,
    #[doc = "Reset caused by JTAG"]
    _1,
}
impl SJTAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SJTAGW::_0 => false,
            SJTAGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SJTAGW<'a> {
    w: &'a mut W,
}
impl<'a> _SJTAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SJTAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by JTAG"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SJTAGW::_0)
    }
    #[doc = "Reset caused by JTAG"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SJTAGW::_1)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SLOCKUP`"]
pub enum SLOCKUPW {
    #[doc = "Reset not caused by core LOCKUP event"]
    _0,
    #[doc = "Reset caused by core LOCKUP event"]
    _1,
}
impl SLOCKUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLOCKUPW::_0 => false,
            SLOCKUPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLOCKUPW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOCKUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLOCKUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by core LOCKUP event"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOCKUPW::_0)
    }
    #[doc = "Reset caused by core LOCKUP event"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOCKUPW::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SSW`"]
pub enum SSWW {
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    _0,
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    _1,
}
impl SSWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSWW::_0 => false,
            SSWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSWW<'a> {
    w: &'a mut W,
}
impl<'a> _SSWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSWW::_0)
    }
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSWW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMDM_AP`"]
pub enum SMDM_APW {
    #[doc = "Reset not caused by host debugger system setting of the System Reset Request bit"]
    _0,
    #[doc = "Reset caused by host debugger system setting of the System Reset Request bit"]
    _1,
}
impl SMDM_APW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMDM_APW::_0 => false,
            SMDM_APW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMDM_APW<'a> {
    w: &'a mut W,
}
impl<'a> _SMDM_APW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMDM_APW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by host debugger system setting of the System Reset Request bit"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMDM_APW::_0)
    }
    #[doc = "Reset caused by host debugger system setting of the System Reset Request bit"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMDM_APW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SEZPT`"]
pub enum SEZPTW {
    #[doc = "Reset not caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    _0,
    #[doc = "Reset caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    _1,
}
impl SEZPTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEZPTW::_0 => false,
            SEZPTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEZPTW<'a> {
    w: &'a mut W,
}
impl<'a> _SEZPTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEZPTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEZPTW::_0)
    }
    #[doc = "Reset caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEZPTW::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SSACKERR`"]
pub enum SSACKERRW {
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _0,
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _1,
}
impl SSACKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSACKERRW::_0 => false,
            SSACKERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSACKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SSACKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSACKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSACKERRW::_0)
    }
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSACKERRW::_1)
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Sticky JTAG Generated Reset"]
    #[inline]
    pub fn sjtag(&self) -> SJTAGR {
        SJTAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Sticky Core Lockup"]
    #[inline]
    pub fn slockup(&self) -> SLOCKUPR {
        SLOCKUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Sticky Software"]
    #[inline]
    pub fn ssw(&self) -> SSWR {
        SSWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Sticky MDM-AP System Reset Request"]
    #[inline]
    pub fn smdm_ap(&self) -> SMDM_APR {
        SMDM_APR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 4 - Sticky EzPort Reset"]
    #[inline]
    pub fn sezpt(&self) -> SEZPTR {
        SEZPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Sticky Stop Mode Acknowledge Error Reset"]
    #[inline]
    pub fn ssackerr(&self) -> SSACKERRR {
        SSACKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Sticky JTAG Generated Reset"]
    #[inline]
    pub fn sjtag(&mut self) -> _SJTAGW {
        _SJTAGW { w: self }
    }
    #[doc = "Bit 1 - Sticky Core Lockup"]
    #[inline]
    pub fn slockup(&mut self) -> _SLOCKUPW {
        _SLOCKUPW { w: self }
    }
    #[doc = "Bit 2 - Sticky Software"]
    #[inline]
    pub fn ssw(&mut self) -> _SSWW {
        _SSWW { w: self }
    }
    #[doc = "Bit 3 - Sticky MDM-AP System Reset Request"]
    #[inline]
    pub fn smdm_ap(&mut self) -> _SMDM_APW {
        _SMDM_APW { w: self }
    }
    #[doc = "Bit 4 - Sticky EzPort Reset"]
    #[inline]
    pub fn sezpt(&mut self) -> _SEZPTW {
        _SEZPTW { w: self }
    }
    #[doc = "Bit 5 - Sticky Stop Mode Acknowledge Error Reset"]
    #[inline]
    pub fn ssackerr(&mut self) -> _SSACKERRW {
        _SSACKERRW { w: self }
    }
}
