#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONF {
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
pub struct NUMTOFR {
    bits: u8,
}
impl NUMTOFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BDMMODER {
    bits: u8,
}
impl BDMMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `GTBEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBEENR {
    #[doc = "Use of an external global time base is disabled."]
    _0,
    #[doc = "Use of an external global time base is enabled."]
    _1,
}
impl GTBEENR {
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
            GTBEENR::_0 => false,
            GTBEENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GTBEENR {
        match value {
            false => GTBEENR::_0,
            true => GTBEENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GTBEENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GTBEENR::_1
    }
}
#[doc = "Possible values of the field `GTBEOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GTBEOUTR {
    #[doc = "A global time base signal generation is disabled."]
    _0,
    #[doc = "A global time base signal generation is enabled."]
    _1,
}
impl GTBEOUTR {
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
            GTBEOUTR::_0 => false,
            GTBEOUTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GTBEOUTR {
        match value {
            false => GTBEOUTR::_0,
            true => GTBEOUTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GTBEOUTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GTBEOUTR::_1
    }
}
#[doc = r" Proxy"]
pub struct _NUMTOFW<'a> {
    w: &'a mut W,
}
impl<'a> _NUMTOFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BDMMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BDMMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GTBEEN`"]
pub enum GTBEENW {
    #[doc = "Use of an external global time base is disabled."]
    _0,
    #[doc = "Use of an external global time base is enabled."]
    _1,
}
impl GTBEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GTBEENW::_0 => false,
            GTBEENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GTBEENW<'a> {
    w: &'a mut W,
}
impl<'a> _GTBEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GTBEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use of an external global time base is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBEENW::_0)
    }
    #[doc = "Use of an external global time base is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBEENW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GTBEOUT`"]
pub enum GTBEOUTW {
    #[doc = "A global time base signal generation is disabled."]
    _0,
    #[doc = "A global time base signal generation is enabled."]
    _1,
}
impl GTBEOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GTBEOUTW::_0 => false,
            GTBEOUTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GTBEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _GTBEOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GTBEOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A global time base signal generation is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBEOUTW::_0)
    }
    #[doc = "A global time base signal generation is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBEOUTW::_1)
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
        const OFFSET: u8 = 10;
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
    #[doc = "Bits 0:4 - TOF Frequency"]
    #[inline]
    pub fn numtof(&self) -> NUMTOFR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUMTOFR { bits }
    }
    #[doc = "Bits 6:7 - BDM Mode"]
    #[inline]
    pub fn bdmmode(&self) -> BDMMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BDMMODER { bits }
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline]
    pub fn gtbeen(&self) -> GTBEENR {
        GTBEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline]
    pub fn gtbeout(&self) -> GTBEOUTR {
        GTBEOUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - TOF Frequency"]
    #[inline]
    pub fn numtof(&mut self) -> _NUMTOFW {
        _NUMTOFW { w: self }
    }
    #[doc = "Bits 6:7 - BDM Mode"]
    #[inline]
    pub fn bdmmode(&mut self) -> _BDMMODEW {
        _BDMMODEW { w: self }
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline]
    pub fn gtbeen(&mut self) -> _GTBEENW {
        _GTBEENW { w: self }
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline]
    pub fn gtbeout(&mut self) -> _GTBEOUTW {
        _GTBEOUTW { w: self }
    }
}
