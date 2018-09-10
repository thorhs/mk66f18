#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RSER {
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
#[doc = "Possible values of the field `RFDF_DIRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFDF_DIRSR {
    #[doc = "Interrupt request."]
    _0,
    #[doc = "DMA request."]
    _1,
}
impl RFDF_DIRSR {
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
            RFDF_DIRSR::_0 => false,
            RFDF_DIRSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFDF_DIRSR {
        match value {
            false => RFDF_DIRSR::_0,
            true => RFDF_DIRSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RFDF_DIRSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RFDF_DIRSR::_1
    }
}
#[doc = "Possible values of the field `RFDF_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFDF_RER {
    #[doc = "RFDF interrupt or DMA requests are disabled."]
    _0,
    #[doc = "RFDF interrupt or DMA requests are enabled."]
    _1,
}
impl RFDF_RER {
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
            RFDF_RER::_0 => false,
            RFDF_RER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFDF_RER {
        match value {
            false => RFDF_RER::_0,
            true => RFDF_RER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RFDF_RER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RFDF_RER::_1
    }
}
#[doc = "Possible values of the field `RFOF_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RFOF_RER {
    #[doc = "RFOF interrupt requests are disabled."]
    _0,
    #[doc = "RFOF interrupt requests are enabled."]
    _1,
}
impl RFOF_RER {
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
            RFOF_RER::_0 => false,
            RFOF_RER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RFOF_RER {
        match value {
            false => RFOF_RER::_0,
            true => RFOF_RER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RFOF_RER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RFOF_RER::_1
    }
}
#[doc = "Possible values of the field `TFFF_DIRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFF_DIRSR {
    #[doc = "TFFF flag generates interrupt requests."]
    _0,
    #[doc = "TFFF flag generates DMA requests."]
    _1,
}
impl TFFF_DIRSR {
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
            TFFF_DIRSR::_0 => false,
            TFFF_DIRSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFFF_DIRSR {
        match value {
            false => TFFF_DIRSR::_0,
            true => TFFF_DIRSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TFFF_DIRSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TFFF_DIRSR::_1
    }
}
#[doc = "Possible values of the field `TFFF_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFFF_RER {
    #[doc = "TFFF interrupts or DMA requests are disabled."]
    _0,
    #[doc = "TFFF interrupts or DMA requests are enabled."]
    _1,
}
impl TFFF_RER {
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
            TFFF_RER::_0 => false,
            TFFF_RER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFFF_RER {
        match value {
            false => TFFF_RER::_0,
            true => TFFF_RER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TFFF_RER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TFFF_RER::_1
    }
}
#[doc = "Possible values of the field `TFUF_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFUF_RER {
    #[doc = "TFUF interrupt requests are disabled."]
    _0,
    #[doc = "TFUF interrupt requests are enabled."]
    _1,
}
impl TFUF_RER {
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
            TFUF_RER::_0 => false,
            TFUF_RER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFUF_RER {
        match value {
            false => TFUF_RER::_0,
            true => TFUF_RER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TFUF_RER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TFUF_RER::_1
    }
}
#[doc = "Possible values of the field `EOQF_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOQF_RER {
    #[doc = "EOQF interrupt requests are disabled."]
    _0,
    #[doc = "EOQF interrupt requests are enabled."]
    _1,
}
impl EOQF_RER {
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
            EOQF_RER::_0 => false,
            EOQF_RER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOQF_RER {
        match value {
            false => EOQF_RER::_0,
            true => EOQF_RER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EOQF_RER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EOQF_RER::_1
    }
}
#[doc = "Possible values of the field `TCF_RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCF_RER {
    #[doc = "TCF interrupt requests are disabled."]
    _0,
    #[doc = "TCF interrupt requests are enabled."]
    _1,
}
impl TCF_RER {
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
            TCF_RER::_0 => false,
            TCF_RER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCF_RER {
        match value {
            false => TCF_RER::_0,
            true => TCF_RER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCF_RER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCF_RER::_1
    }
}
#[doc = "Values that can be written to the field `RFDF_DIRS`"]
pub enum RFDF_DIRSW {
    #[doc = "Interrupt request."]
    _0,
    #[doc = "DMA request."]
    _1,
}
impl RFDF_DIRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFDF_DIRSW::_0 => false,
            RFDF_DIRSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFDF_DIRSW<'a> {
    w: &'a mut W,
}
impl<'a> _RFDF_DIRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFDF_DIRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt request."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFDF_DIRSW::_0)
    }
    #[doc = "DMA request."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFDF_DIRSW::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RFDF_RE`"]
pub enum RFDF_REW {
    #[doc = "RFDF interrupt or DMA requests are disabled."]
    _0,
    #[doc = "RFDF interrupt or DMA requests are enabled."]
    _1,
}
impl RFDF_REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFDF_REW::_0 => false,
            RFDF_REW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFDF_REW<'a> {
    w: &'a mut W,
}
impl<'a> _RFDF_REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFDF_REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RFDF interrupt or DMA requests are disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFDF_REW::_0)
    }
    #[doc = "RFDF interrupt or DMA requests are enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFDF_REW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RFOF_RE`"]
pub enum RFOF_REW {
    #[doc = "RFOF interrupt requests are disabled."]
    _0,
    #[doc = "RFOF interrupt requests are enabled."]
    _1,
}
impl RFOF_REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RFOF_REW::_0 => false,
            RFOF_REW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RFOF_REW<'a> {
    w: &'a mut W,
}
impl<'a> _RFOF_REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RFOF_REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RFOF interrupt requests are disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFOF_REW::_0)
    }
    #[doc = "RFOF interrupt requests are enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFOF_REW::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TFFF_DIRS`"]
pub enum TFFF_DIRSW {
    #[doc = "TFFF flag generates interrupt requests."]
    _0,
    #[doc = "TFFF flag generates DMA requests."]
    _1,
}
impl TFFF_DIRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFFF_DIRSW::_0 => false,
            TFFF_DIRSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFFF_DIRSW<'a> {
    w: &'a mut W,
}
impl<'a> _TFFF_DIRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFFF_DIRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TFFF flag generates interrupt requests."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFFF_DIRSW::_0)
    }
    #[doc = "TFFF flag generates DMA requests."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFFF_DIRSW::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TFFF_RE`"]
pub enum TFFF_REW {
    #[doc = "TFFF interrupts or DMA requests are disabled."]
    _0,
    #[doc = "TFFF interrupts or DMA requests are enabled."]
    _1,
}
impl TFFF_REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFFF_REW::_0 => false,
            TFFF_REW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFFF_REW<'a> {
    w: &'a mut W,
}
impl<'a> _TFFF_REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFFF_REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TFFF interrupts or DMA requests are disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFFF_REW::_0)
    }
    #[doc = "TFFF interrupts or DMA requests are enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFFF_REW::_1)
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
#[doc = "Values that can be written to the field `TFUF_RE`"]
pub enum TFUF_REW {
    #[doc = "TFUF interrupt requests are disabled."]
    _0,
    #[doc = "TFUF interrupt requests are enabled."]
    _1,
}
impl TFUF_REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFUF_REW::_0 => false,
            TFUF_REW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFUF_REW<'a> {
    w: &'a mut W,
}
impl<'a> _TFUF_REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFUF_REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TFUF interrupt requests are disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFUF_REW::_0)
    }
    #[doc = "TFUF interrupt requests are enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFUF_REW::_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EOQF_RE`"]
pub enum EOQF_REW {
    #[doc = "EOQF interrupt requests are disabled."]
    _0,
    #[doc = "EOQF interrupt requests are enabled."]
    _1,
}
impl EOQF_REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOQF_REW::_0 => false,
            EOQF_REW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOQF_REW<'a> {
    w: &'a mut W,
}
impl<'a> _EOQF_REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOQF_REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "EOQF interrupt requests are disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOQF_REW::_0)
    }
    #[doc = "EOQF interrupt requests are enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOQF_REW::_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCF_RE`"]
pub enum TCF_REW {
    #[doc = "TCF interrupt requests are disabled."]
    _0,
    #[doc = "TCF interrupt requests are enabled."]
    _1,
}
impl TCF_REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCF_REW::_0 => false,
            TCF_REW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCF_REW<'a> {
    w: &'a mut W,
}
impl<'a> _TCF_REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCF_REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TCF interrupt requests are disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCF_REW::_0)
    }
    #[doc = "TCF interrupt requests are enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCF_REW::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 16 - Receive FIFO Drain DMA or Interrupt Request Select"]
    #[inline]
    pub fn rfdf_dirs(&self) -> RFDF_DIRSR {
        RFDF_DIRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Receive FIFO Drain Request Enable"]
    #[inline]
    pub fn rfdf_re(&self) -> RFDF_RER {
        RFDF_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Request Enable"]
    #[inline]
    pub fn rfof_re(&self) -> RFOF_RER {
        RFOF_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Transmit FIFO Fill DMA or Interrupt Request Select"]
    #[inline]
    pub fn tfff_dirs(&self) -> TFFF_DIRSR {
        TFFF_DIRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Request Enable"]
    #[inline]
    pub fn tfff_re(&self) -> TFFF_RER {
        TFFF_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Request Enable"]
    #[inline]
    pub fn tfuf_re(&self) -> TFUF_RER {
        TFUF_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Finished Request Enable"]
    #[inline]
    pub fn eoqf_re(&self) -> EOQF_RER {
        EOQF_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Transmission Complete Request Enable"]
    #[inline]
    pub fn tcf_re(&self) -> TCF_RER {
        TCF_RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 16 - Receive FIFO Drain DMA or Interrupt Request Select"]
    #[inline]
    pub fn rfdf_dirs(&mut self) -> _RFDF_DIRSW {
        _RFDF_DIRSW { w: self }
    }
    #[doc = "Bit 17 - Receive FIFO Drain Request Enable"]
    #[inline]
    pub fn rfdf_re(&mut self) -> _RFDF_REW {
        _RFDF_REW { w: self }
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Request Enable"]
    #[inline]
    pub fn rfof_re(&mut self) -> _RFOF_REW {
        _RFOF_REW { w: self }
    }
    #[doc = "Bit 24 - Transmit FIFO Fill DMA or Interrupt Request Select"]
    #[inline]
    pub fn tfff_dirs(&mut self) -> _TFFF_DIRSW {
        _TFFF_DIRSW { w: self }
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Request Enable"]
    #[inline]
    pub fn tfff_re(&mut self) -> _TFFF_REW {
        _TFFF_REW { w: self }
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Request Enable"]
    #[inline]
    pub fn tfuf_re(&mut self) -> _TFUF_REW {
        _TFUF_REW { w: self }
    }
    #[doc = "Bit 28 - Finished Request Enable"]
    #[inline]
    pub fn eoqf_re(&mut self) -> _EOQF_REW {
        _EOQF_REW { w: self }
    }
    #[doc = "Bit 31 - Transmission Complete Request Enable"]
    #[inline]
    pub fn tcf_re(&mut self) -> _TCF_REW {
        _TCF_REW { w: self }
    }
}
