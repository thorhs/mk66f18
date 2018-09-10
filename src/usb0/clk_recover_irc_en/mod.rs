#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::CLK_RECOVER_IRC_EN {
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
#[doc = "Possible values of the field `REG_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_ENR {
    #[doc = "IRC48M local regulator is disabled"]
    _0,
    #[doc = "IRC48M local regulator is enabled (default)"]
    _1,
}
impl REG_ENR {
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
            REG_ENR::_0 => false,
            REG_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REG_ENR {
        match value {
            false => REG_ENR::_0,
            true => REG_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == REG_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == REG_ENR::_1
    }
}
#[doc = "Possible values of the field `IRC_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRC_ENR {
    #[doc = "Disable the IRC48M module (default)"]
    _0,
    #[doc = "Enable the IRC48M module"]
    _1,
}
impl IRC_ENR {
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
            IRC_ENR::_0 => false,
            IRC_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IRC_ENR {
        match value {
            false => IRC_ENR::_0,
            true => IRC_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IRC_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IRC_ENR::_1
    }
}
#[doc = "Values that can be written to the field `REG_EN`"]
pub enum REG_ENW {
    #[doc = "IRC48M local regulator is disabled"]
    _0,
    #[doc = "IRC48M local regulator is enabled (default)"]
    _1,
}
impl REG_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REG_ENW::_0 => false,
            REG_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _REG_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REG_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "IRC48M local regulator is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(REG_ENW::_0)
    }
    #[doc = "IRC48M local regulator is enabled (default)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(REG_ENW::_1)
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
#[doc = "Values that can be written to the field `IRC_EN`"]
pub enum IRC_ENW {
    #[doc = "Disable the IRC48M module (default)"]
    _0,
    #[doc = "Enable the IRC48M module"]
    _1,
}
impl IRC_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IRC_ENW::_0 => false,
            IRC_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IRC_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _IRC_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IRC_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the IRC48M module (default)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRC_ENW::_0)
    }
    #[doc = "Enable the IRC48M module"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRC_ENW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - IRC48M regulator enable"]
    #[inline]
    pub fn reg_en(&self) -> REG_ENR {
        REG_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - IRC48M enable"]
    #[inline]
    pub fn irc_en(&self) -> IRC_ENR {
        IRC_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - IRC48M regulator enable"]
    #[inline]
    pub fn reg_en(&mut self) -> _REG_ENW {
        _REG_ENW { w: self }
    }
    #[doc = "Bit 1 - IRC48M enable"]
    #[inline]
    pub fn irc_en(&mut self) -> _IRC_ENW {
        _IRC_ENW { w: self }
    }
}
