#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL2 {
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
#[doc = "Possible values of the field `EACEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EACENR {
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    _0,
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    _1,
}
impl EACENR {
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
            EACENR::_0 => false,
            EACENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EACENR {
        match value {
            false => EACENR::_0,
            true => EACENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EACENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EACENR::_1
    }
}
#[doc = "Possible values of the field `RRS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRSR {
    #[doc = "Remote Response Frame is generated."]
    _0,
    #[doc = "Remote Request Frame is stored."]
    _1,
}
impl RRSR {
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
            RRSR::_0 => false,
            RRSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RRSR {
        match value {
            false => RRSR::_0,
            true => RRSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RRSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RRSR::_1
    }
}
#[doc = "Possible values of the field `MRP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRPR {
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes."]
    _0,
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO."]
    _1,
}
impl MRPR {
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
            MRPR::_0 => false,
            MRPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MRPR {
        match value {
            false => MRPR::_0,
            true => MRPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MRPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MRPR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TASDR {
    bits: u8,
}
impl TASDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RFFNR {
    bits: u8,
}
impl RFFNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WRMFRZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRMFRZR {
    #[doc = "Maintain the write access restrictions."]
    _0,
    #[doc = "Enable unrestricted write access to FlexCAN memory."]
    _1,
}
impl WRMFRZR {
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
            WRMFRZR::_0 => false,
            WRMFRZR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRMFRZR {
        match value {
            false => WRMFRZR::_0,
            true => WRMFRZR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WRMFRZR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WRMFRZR::_1
    }
}
#[doc = "Values that can be written to the field `EACEN`"]
pub enum EACENW {
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    _0,
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    _1,
}
impl EACENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EACENW::_0 => false,
            EACENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EACENW<'a> {
    w: &'a mut W,
}
impl<'a> _EACENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EACENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EACENW::_0)
    }
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EACENW::_1)
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
#[doc = "Values that can be written to the field `RRS`"]
pub enum RRSW {
    #[doc = "Remote Response Frame is generated."]
    _0,
    #[doc = "Remote Request Frame is stored."]
    _1,
}
impl RRSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RRSW::_0 => false,
            RRSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RRSW<'a> {
    w: &'a mut W,
}
impl<'a> _RRSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RRSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Remote Response Frame is generated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RRSW::_0)
    }
    #[doc = "Remote Request Frame is stored."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RRSW::_1)
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
#[doc = "Values that can be written to the field `MRP`"]
pub enum MRPW {
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes."]
    _0,
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO."]
    _1,
}
impl MRPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MRPW::_0 => false,
            MRPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MRPW<'a> {
    w: &'a mut W,
}
impl<'a> _MRPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MRPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MRPW::_0)
    }
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MRPW::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TASDW<'a> {
    w: &'a mut W,
}
impl<'a> _TASDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RFFNW<'a> {
    w: &'a mut W,
}
impl<'a> _RFFNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WRMFRZ`"]
pub enum WRMFRZW {
    #[doc = "Maintain the write access restrictions."]
    _0,
    #[doc = "Enable unrestricted write access to FlexCAN memory."]
    _1,
}
impl WRMFRZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WRMFRZW::_0 => false,
            WRMFRZW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRMFRZW<'a> {
    w: &'a mut W,
}
impl<'a> _WRMFRZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRMFRZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Maintain the write access restrictions."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WRMFRZW::_0)
    }
    #[doc = "Enable unrestricted write access to FlexCAN memory."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WRMFRZW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline]
    pub fn eacen(&self) -> EACENR {
        EACENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline]
    pub fn rrs(&self) -> RRSR {
        RRSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline]
    pub fn mrp(&self) -> MRPR {
        MRPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline]
    pub fn tasd(&self) -> TASDR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TASDR { bits }
    }
    #[doc = "Bits 24:27 - Number Of Rx FIFO Filters"]
    #[inline]
    pub fn rffn(&self) -> RFFNR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RFFNR { bits }
    }
    #[doc = "Bit 28 - Write-Access To Memory In Freeze Mode"]
    #[inline]
    pub fn wrmfrz(&self) -> WRMFRZR {
        WRMFRZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 11534336 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline]
    pub fn eacen(&mut self) -> _EACENW {
        _EACENW { w: self }
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline]
    pub fn rrs(&mut self) -> _RRSW {
        _RRSW { w: self }
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline]
    pub fn mrp(&mut self) -> _MRPW {
        _MRPW { w: self }
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline]
    pub fn tasd(&mut self) -> _TASDW {
        _TASDW { w: self }
    }
    #[doc = "Bits 24:27 - Number Of Rx FIFO Filters"]
    #[inline]
    pub fn rffn(&mut self) -> _RFFNW {
        _RFFNW { w: self }
    }
    #[doc = "Bit 28 - Write-Access To Memory In Freeze Mode"]
    #[inline]
    pub fn wrmfrz(&mut self) -> _WRMFRZW {
        _WRMFRZW { w: self }
    }
}
