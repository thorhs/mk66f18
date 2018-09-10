#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::PMPROT {
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
#[doc = "Possible values of the field `AVLLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVLLSR {
    #[doc = "Any VLLSx mode is not allowed"]
    _0,
    #[doc = "Any VLLSx mode is allowed"]
    _1,
}
impl AVLLSR {
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
            AVLLSR::_0 => false,
            AVLLSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVLLSR {
        match value {
            false => AVLLSR::_0,
            true => AVLLSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AVLLSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AVLLSR::_1
    }
}
#[doc = "Possible values of the field `ALLS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALLSR {
    #[doc = "Any LLSx mode is not allowed"]
    _0,
    #[doc = "Any LLSx mode is allowed"]
    _1,
}
impl ALLSR {
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
            ALLSR::_0 => false,
            ALLSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALLSR {
        match value {
            false => ALLSR::_0,
            true => ALLSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ALLSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ALLSR::_1
    }
}
#[doc = "Possible values of the field `AVLP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVLPR {
    #[doc = "VLPR, VLPW, and VLPS are not allowed."]
    _0,
    #[doc = "VLPR, VLPW, and VLPS are allowed."]
    _1,
}
impl AVLPR {
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
            AVLPR::_0 => false,
            AVLPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVLPR {
        match value {
            false => AVLPR::_0,
            true => AVLPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AVLPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AVLPR::_1
    }
}
#[doc = "Possible values of the field `AHSRUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHSRUNR {
    #[doc = "HSRUN is not allowed"]
    _0,
    #[doc = "HSRUN is allowed"]
    _1,
}
impl AHSRUNR {
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
            AHSRUNR::_0 => false,
            AHSRUNR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AHSRUNR {
        match value {
            false => AHSRUNR::_0,
            true => AHSRUNR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AHSRUNR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AHSRUNR::_1
    }
}
#[doc = "Values that can be written to the field `AVLLS`"]
pub enum AVLLSW {
    #[doc = "Any VLLSx mode is not allowed"]
    _0,
    #[doc = "Any VLLSx mode is allowed"]
    _1,
}
impl AVLLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AVLLSW::_0 => false,
            AVLLSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVLLSW<'a> {
    w: &'a mut W,
}
impl<'a> _AVLLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVLLSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Any VLLSx mode is not allowed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVLLSW::_0)
    }
    #[doc = "Any VLLSx mode is allowed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVLLSW::_1)
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
#[doc = "Values that can be written to the field `ALLS`"]
pub enum ALLSW {
    #[doc = "Any LLSx mode is not allowed"]
    _0,
    #[doc = "Any LLSx mode is allowed"]
    _1,
}
impl ALLSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALLSW::_0 => false,
            ALLSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALLSW<'a> {
    w: &'a mut W,
}
impl<'a> _ALLSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALLSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Any LLSx mode is not allowed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALLSW::_0)
    }
    #[doc = "Any LLSx mode is allowed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALLSW::_1)
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
#[doc = "Values that can be written to the field `AVLP`"]
pub enum AVLPW {
    #[doc = "VLPR, VLPW, and VLPS are not allowed."]
    _0,
    #[doc = "VLPR, VLPW, and VLPS are allowed."]
    _1,
}
impl AVLPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AVLPW::_0 => false,
            AVLPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AVLPW<'a> {
    w: &'a mut W,
}
impl<'a> _AVLPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AVLPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "VLPR, VLPW, and VLPS are not allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVLPW::_0)
    }
    #[doc = "VLPR, VLPW, and VLPS are allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVLPW::_1)
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
#[doc = "Values that can be written to the field `AHSRUN`"]
pub enum AHSRUNW {
    #[doc = "HSRUN is not allowed"]
    _0,
    #[doc = "HSRUN is allowed"]
    _1,
}
impl AHSRUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            AHSRUNW::_0 => false,
            AHSRUNW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AHSRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _AHSRUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AHSRUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "HSRUN is not allowed"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(AHSRUNW::_0)
    }
    #[doc = "HSRUN is allowed"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(AHSRUNW::_1)
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
    #[doc = "Bit 1 - Allow Very-Low-Leakage Stop Mode"]
    #[inline]
    pub fn avlls(&self) -> AVLLSR {
        AVLLSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 3 - Allow Low-Leakage Stop Mode"]
    #[inline]
    pub fn alls(&self) -> ALLSR {
        ALLSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline]
    pub fn avlp(&self) -> AVLPR {
        AVLPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Allow High Speed Run mode"]
    #[inline]
    pub fn ahsrun(&self) -> AHSRUNR {
        AHSRUNR::_from({
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
    #[doc = "Bit 1 - Allow Very-Low-Leakage Stop Mode"]
    #[inline]
    pub fn avlls(&mut self) -> _AVLLSW {
        _AVLLSW { w: self }
    }
    #[doc = "Bit 3 - Allow Low-Leakage Stop Mode"]
    #[inline]
    pub fn alls(&mut self) -> _ALLSW {
        _ALLSW { w: self }
    }
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline]
    pub fn avlp(&mut self) -> _AVLPW {
        _AVLPW { w: self }
    }
    #[doc = "Bit 7 - Allow High Speed Run mode"]
    #[inline]
    pub fn ahsrun(&mut self) -> _AHSRUNW {
        _AHSRUNW { w: self }
    }
}
