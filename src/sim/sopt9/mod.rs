#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SOPT9 {
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
#[doc = "Possible values of the field `TPM1CH0SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM1CH0SRCR {
    #[doc = "TPM1_CH0 signal"]
    _00,
    #[doc = "CMP0 output"]
    _01,
    #[doc = "CMP1 output"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TPM1CH0SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TPM1CH0SRCR::_00 => 0,
            TPM1CH0SRCR::_01 => 1,
            TPM1CH0SRCR::_10 => 2,
            TPM1CH0SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TPM1CH0SRCR {
        match value {
            0 => TPM1CH0SRCR::_00,
            1 => TPM1CH0SRCR::_01,
            2 => TPM1CH0SRCR::_10,
            i => TPM1CH0SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TPM1CH0SRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TPM1CH0SRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TPM1CH0SRCR::_10
    }
}
#[doc = "Possible values of the field `TPM2CH0SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM2CH0SRCR {
    #[doc = "TPM2_CH0 signal"]
    _00,
    #[doc = "CMP0 output"]
    _01,
    #[doc = "CMP1 output"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TPM2CH0SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TPM2CH0SRCR::_00 => 0,
            TPM2CH0SRCR::_01 => 1,
            TPM2CH0SRCR::_10 => 2,
            TPM2CH0SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TPM2CH0SRCR {
        match value {
            0 => TPM2CH0SRCR::_00,
            1 => TPM2CH0SRCR::_01,
            2 => TPM2CH0SRCR::_10,
            i => TPM2CH0SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TPM2CH0SRCR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TPM2CH0SRCR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TPM2CH0SRCR::_10
    }
}
#[doc = "Possible values of the field `TPM1CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM1CLKSELR {
    #[doc = "TPM_CLKIN0 pin"]
    _0,
    #[doc = "TPM_CLKIN1 pin"]
    _1,
}
impl TPM1CLKSELR {
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
            TPM1CLKSELR::_0 => false,
            TPM1CLKSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPM1CLKSELR {
        match value {
            false => TPM1CLKSELR::_0,
            true => TPM1CLKSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TPM1CLKSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TPM1CLKSELR::_1
    }
}
#[doc = "Possible values of the field `TPM2CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM2CLKSELR {
    #[doc = "TPM_CLKIN0 pin"]
    _0,
    #[doc = "TPM_CLKIN1 pin"]
    _1,
}
impl TPM2CLKSELR {
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
            TPM2CLKSELR::_0 => false,
            TPM2CLKSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPM2CLKSELR {
        match value {
            false => TPM2CLKSELR::_0,
            true => TPM2CLKSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TPM2CLKSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TPM2CLKSELR::_1
    }
}
#[doc = "Values that can be written to the field `TPM1CH0SRC`"]
pub enum TPM1CH0SRCW {
    #[doc = "TPM1_CH0 signal"]
    _00,
    #[doc = "CMP0 output"]
    _01,
    #[doc = "CMP1 output"]
    _10,
}
impl TPM1CH0SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TPM1CH0SRCW::_00 => 0,
            TPM1CH0SRCW::_01 => 1,
            TPM1CH0SRCW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPM1CH0SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TPM1CH0SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPM1CH0SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TPM1_CH0 signal"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPM1CH0SRCW::_00)
    }
    #[doc = "CMP0 output"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPM1CH0SRCW::_01)
    }
    #[doc = "CMP1 output"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPM1CH0SRCW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPM2CH0SRC`"]
pub enum TPM2CH0SRCW {
    #[doc = "TPM2_CH0 signal"]
    _00,
    #[doc = "CMP0 output"]
    _01,
    #[doc = "CMP1 output"]
    _10,
}
impl TPM2CH0SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TPM2CH0SRCW::_00 => 0,
            TPM2CH0SRCW::_01 => 1,
            TPM2CH0SRCW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPM2CH0SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TPM2CH0SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPM2CH0SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "TPM2_CH0 signal"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPM2CH0SRCW::_00)
    }
    #[doc = "CMP0 output"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPM2CH0SRCW::_01)
    }
    #[doc = "CMP1 output"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPM2CH0SRCW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPM1CLKSEL`"]
pub enum TPM1CLKSELW {
    #[doc = "TPM_CLKIN0 pin"]
    _0,
    #[doc = "TPM_CLKIN1 pin"]
    _1,
}
impl TPM1CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPM1CLKSELW::_0 => false,
            TPM1CLKSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPM1CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TPM1CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPM1CLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TPM_CLKIN0 pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM1CLKSELW::_0)
    }
    #[doc = "TPM_CLKIN1 pin"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM1CLKSELW::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPM2CLKSEL`"]
pub enum TPM2CLKSELW {
    #[doc = "TPM_CLKIN0 pin"]
    _0,
    #[doc = "TPM_CLKIN1 pin"]
    _1,
}
impl TPM2CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPM2CLKSELW::_0 => false,
            TPM2CLKSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPM2CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TPM2CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPM2CLKSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TPM_CLKIN0 pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPM2CLKSELW::_0)
    }
    #[doc = "TPM_CLKIN1 pin"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPM2CLKSELW::_1)
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
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 18:19 - TPM1 channel 0 input capture source select"]
    #[inline]
    pub fn tpm1ch0src(&self) -> TPM1CH0SRCR {
        TPM1CH0SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - TPM2 channel 0 input capture source select"]
    #[inline]
    pub fn tpm2ch0src(&self) -> TPM2CH0SRCR {
        TPM2CH0SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 25 - TPM1 External Clock Pin Select"]
    #[inline]
    pub fn tpm1clksel(&self) -> TPM1CLKSELR {
        TPM1CLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - TPM2 External Clock Pin Select"]
    #[inline]
    pub fn tpm2clksel(&self) -> TPM2CLKSELR {
        TPM2CLKSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
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
    #[doc = "Bits 18:19 - TPM1 channel 0 input capture source select"]
    #[inline]
    pub fn tpm1ch0src(&mut self) -> _TPM1CH0SRCW {
        _TPM1CH0SRCW { w: self }
    }
    #[doc = "Bits 20:21 - TPM2 channel 0 input capture source select"]
    #[inline]
    pub fn tpm2ch0src(&mut self) -> _TPM2CH0SRCW {
        _TPM2CH0SRCW { w: self }
    }
    #[doc = "Bit 25 - TPM1 External Clock Pin Select"]
    #[inline]
    pub fn tpm1clksel(&mut self) -> _TPM1CLKSELW {
        _TPM1CLKSELW { w: self }
    }
    #[doc = "Bit 26 - TPM2 External Clock Pin Select"]
    #[inline]
    pub fn tpm2clksel(&mut self) -> _TPM2CLKSELW {
        _TPM2CLKSELW { w: self }
    }
}
