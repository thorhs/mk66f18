#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CM {
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
#[doc = "Possible values of the field `V`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VR {
    #[doc = "Do not decode DRAM accesses."]
    _0,
    #[doc = "Registers controlling the DRAM block are initialized; DRAM accesses can be decoded"]
    _1,
}
impl VR {
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
            VR::_0 => false,
            VR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VR {
        match value {
            false => VR::_0,
            true => VR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VR::_1
    }
}
#[doc = "Possible values of the field `WP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPR {
    #[doc = "Allow write accesses"]
    _0,
    #[doc = "Ignore write accesses. The DRAM controller ignores write accesses to the memory block and an address exception occurs. Write accesses to a write-protected DRAM region are compared in the chip select module for a hit. If no hit occurs, an external bus cycle is generated. If this external bus cycle is not acknowledged, an access exception occurs."]
    _1,
}
impl WPR {
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
            WPR::_0 => false,
            WPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WPR {
        match value {
            false => WPR::_0,
            true => WPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WPR::_1
    }
}
#[doc = "Possible values of the field `BAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BAMR {
    #[doc = "The associated address bit is used in decoding the DRAM hit to a memory block"]
    _0,
    #[doc = "The associated address bit is not used in the DRAM hit decode"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl BAMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            BAMR::_0 => 0,
            BAMR::_1 => 1,
            BAMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> BAMR {
        match value {
            0 => BAMR::_0,
            1 => BAMR::_1,
            i => BAMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BAMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BAMR::_1
    }
}
#[doc = "Values that can be written to the field `V`"]
pub enum VW {
    #[doc = "Do not decode DRAM accesses."]
    _0,
    #[doc = "Registers controlling the DRAM block are initialized; DRAM accesses can be decoded"]
    _1,
}
impl VW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VW::_0 => false,
            VW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VW<'a> {
    w: &'a mut W,
}
impl<'a> _VW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not decode DRAM accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(VW::_0)
    }
    #[doc = "Registers controlling the DRAM block are initialized; DRAM accesses can be decoded"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(VW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WP`"]
pub enum WPW {
    #[doc = "Allow write accesses"]
    _0,
    #[doc = "Ignore write accesses. The DRAM controller ignores write accesses to the memory block and an address exception occurs. Write accesses to a write-protected DRAM region are compared in the chip select module for a hit. If no hit occurs, an external bus cycle is generated. If this external bus cycle is not acknowledged, an access exception occurs."]
    _1,
}
impl WPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WPW::_0 => false,
            WPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WPW<'a> {
    w: &'a mut W,
}
impl<'a> _WPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Allow write accesses"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPW::_0)
    }
    #[doc = "Ignore write accesses. The DRAM controller ignores write accesses to the memory block and an address exception occurs. Write accesses to a write-protected DRAM region are compared in the chip select module for a hit. If no hit occurs, an external bus cycle is generated. If this external bus cycle is not acknowledged, an access exception occurs."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPW::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BAM`"]
pub enum BAMW {
    #[doc = "The associated address bit is used in decoding the DRAM hit to a memory block"]
    _0,
    #[doc = "The associated address bit is not used in the DRAM hit decode"]
    _1,
}
impl BAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            BAMW::_0 => 0,
            BAMW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BAMW<'a> {
    w: &'a mut W,
}
impl<'a> _BAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BAMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The associated address bit is used in decoding the DRAM hit to a memory block"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BAMW::_0)
    }
    #[doc = "The associated address bit is not used in the DRAM hit decode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BAMW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 16383;
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Valid."]
    #[inline]
    pub fn v(&self) -> VR {
        VR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Write protect."]
    #[inline]
    pub fn wp(&self) -> WPR {
        WPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:31 - Base address mask."]
    #[inline]
    pub fn bam(&self) -> BAMR {
        BAMR::_from({
            const MASK: u16 = 16383;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bit 0 - Valid."]
    #[inline]
    pub fn v(&mut self) -> _VW {
        _VW { w: self }
    }
    #[doc = "Bit 8 - Write protect."]
    #[inline]
    pub fn wp(&mut self) -> _WPW {
        _WPW { w: self }
    }
    #[doc = "Bits 18:31 - Base address mask."]
    #[inline]
    pub fn bam(&mut self) -> _BAMW {
        _BAMW { w: self }
    }
}
