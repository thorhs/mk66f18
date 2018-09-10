#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSPMCR {
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
#[doc = "Possible values of the field `GROUP5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP5R {
    #[doc = "FB_TA"]
    _0000,
    #[doc = "FB_CS3 . You must also write 1b to CSCR\\[AA\\]."]
    _0001,
    #[doc = "FB_BE_7_0 . You must also write 1b to CSCR\\[AA\\]."]
    _0010,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GROUP5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GROUP5R::_0000 => 0,
            GROUP5R::_0001 => 1,
            GROUP5R::_0010 => 2,
            GROUP5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GROUP5R {
        match value {
            0 => GROUP5R::_0000,
            1 => GROUP5R::_0001,
            2 => GROUP5R::_0010,
            i => GROUP5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == GROUP5R::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == GROUP5R::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == GROUP5R::_0010
    }
}
#[doc = "Possible values of the field `GROUP4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP4R {
    #[doc = "FB_TBST"]
    _0000,
    #[doc = "FB_CS2"]
    _0001,
    #[doc = "FB_BE_15_8"]
    _0010,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GROUP4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GROUP4R::_0000 => 0,
            GROUP4R::_0001 => 1,
            GROUP4R::_0010 => 2,
            GROUP4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GROUP4R {
        match value {
            0 => GROUP4R::_0000,
            1 => GROUP4R::_0001,
            2 => GROUP4R::_0010,
            i => GROUP4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == GROUP4R::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == GROUP4R::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == GROUP4R::_0010
    }
}
#[doc = "Possible values of the field `GROUP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP3R {
    #[doc = "FB_CS5"]
    _0000,
    #[doc = "FB_TSIZ1"]
    _0001,
    #[doc = "FB_BE_23_16"]
    _0010,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GROUP3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GROUP3R::_0000 => 0,
            GROUP3R::_0001 => 1,
            GROUP3R::_0010 => 2,
            GROUP3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GROUP3R {
        match value {
            0 => GROUP3R::_0000,
            1 => GROUP3R::_0001,
            2 => GROUP3R::_0010,
            i => GROUP3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == GROUP3R::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == GROUP3R::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == GROUP3R::_0010
    }
}
#[doc = "Possible values of the field `GROUP2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP2R {
    #[doc = "FB_CS4"]
    _0000,
    #[doc = "FB_TSIZ0"]
    _0001,
    #[doc = "FB_BE_31_24"]
    _0010,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GROUP2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GROUP2R::_0000 => 0,
            GROUP2R::_0001 => 1,
            GROUP2R::_0010 => 2,
            GROUP2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GROUP2R {
        match value {
            0 => GROUP2R::_0000,
            1 => GROUP2R::_0001,
            2 => GROUP2R::_0010,
            i => GROUP2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == GROUP2R::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == GROUP2R::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == GROUP2R::_0010
    }
}
#[doc = "Possible values of the field `GROUP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP1R {
    #[doc = "FB_ALE"]
    _0000,
    #[doc = "FB_CS1"]
    _0001,
    #[doc = "FB_TS"]
    _0010,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl GROUP1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            GROUP1R::_0000 => 0,
            GROUP1R::_0001 => 1,
            GROUP1R::_0010 => 2,
            GROUP1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> GROUP1R {
        match value {
            0 => GROUP1R::_0000,
            1 => GROUP1R::_0001,
            2 => GROUP1R::_0010,
            i => GROUP1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == GROUP1R::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == GROUP1R::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == GROUP1R::_0010
    }
}
#[doc = "Values that can be written to the field `GROUP5`"]
pub enum GROUP5W {
    #[doc = "FB_TA"]
    _0000,
    #[doc = "FB_CS3 . You must also write 1b to CSCR\\[AA\\]."]
    _0001,
    #[doc = "FB_BE_7_0 . You must also write 1b to CSCR\\[AA\\]."]
    _0010,
}
impl GROUP5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GROUP5W::_0000 => 0,
            GROUP5W::_0001 => 1,
            GROUP5W::_0010 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GROUP5W<'a> {
    w: &'a mut W,
}
impl<'a> _GROUP5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GROUP5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FB_TA"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP5W::_0000)
    }
    #[doc = "FB_CS3 . You must also write 1b to CSCR\\[AA\\]."]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP5W::_0001)
    }
    #[doc = "FB_BE_7_0 . You must also write 1b to CSCR\\[AA\\]."]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP5W::_0010)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GROUP4`"]
pub enum GROUP4W {
    #[doc = "FB_TBST"]
    _0000,
    #[doc = "FB_CS2"]
    _0001,
    #[doc = "FB_BE_15_8"]
    _0010,
}
impl GROUP4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GROUP4W::_0000 => 0,
            GROUP4W::_0001 => 1,
            GROUP4W::_0010 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GROUP4W<'a> {
    w: &'a mut W,
}
impl<'a> _GROUP4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GROUP4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FB_TBST"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP4W::_0000)
    }
    #[doc = "FB_CS2"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP4W::_0001)
    }
    #[doc = "FB_BE_15_8"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP4W::_0010)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GROUP3`"]
pub enum GROUP3W {
    #[doc = "FB_CS5"]
    _0000,
    #[doc = "FB_TSIZ1"]
    _0001,
    #[doc = "FB_BE_23_16"]
    _0010,
}
impl GROUP3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GROUP3W::_0000 => 0,
            GROUP3W::_0001 => 1,
            GROUP3W::_0010 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GROUP3W<'a> {
    w: &'a mut W,
}
impl<'a> _GROUP3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GROUP3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FB_CS5"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP3W::_0000)
    }
    #[doc = "FB_TSIZ1"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP3W::_0001)
    }
    #[doc = "FB_BE_23_16"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP3W::_0010)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GROUP2`"]
pub enum GROUP2W {
    #[doc = "FB_CS4"]
    _0000,
    #[doc = "FB_TSIZ0"]
    _0001,
    #[doc = "FB_BE_31_24"]
    _0010,
}
impl GROUP2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GROUP2W::_0000 => 0,
            GROUP2W::_0001 => 1,
            GROUP2W::_0010 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GROUP2W<'a> {
    w: &'a mut W,
}
impl<'a> _GROUP2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GROUP2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FB_CS4"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP2W::_0000)
    }
    #[doc = "FB_TSIZ0"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP2W::_0001)
    }
    #[doc = "FB_BE_31_24"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP2W::_0010)
    }
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
#[doc = "Values that can be written to the field `GROUP1`"]
pub enum GROUP1W {
    #[doc = "FB_ALE"]
    _0000,
    #[doc = "FB_CS1"]
    _0001,
    #[doc = "FB_TS"]
    _0010,
}
impl GROUP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            GROUP1W::_0000 => 0,
            GROUP1W::_0001 => 1,
            GROUP1W::_0010 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GROUP1W<'a> {
    w: &'a mut W,
}
impl<'a> _GROUP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GROUP1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "FB_ALE"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(GROUP1W::_0000)
    }
    #[doc = "FB_CS1"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(GROUP1W::_0001)
    }
    #[doc = "FB_TS"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(GROUP1W::_0010)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 12:15 - FlexBus Signal Group 5 Multiplex control"]
    #[inline]
    pub fn group5(&self) -> GROUP5R {
        GROUP5R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - FlexBus Signal Group 4 Multiplex control"]
    #[inline]
    pub fn group4(&self) -> GROUP4R {
        GROUP4R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - FlexBus Signal Group 3 Multiplex control"]
    #[inline]
    pub fn group3(&self) -> GROUP3R {
        GROUP3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - FlexBus Signal Group 2 Multiplex control"]
    #[inline]
    pub fn group2(&self) -> GROUP2R {
        GROUP2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - FlexBus Signal Group 1 Multiplex control"]
    #[inline]
    pub fn group1(&self) -> GROUP1R {
        GROUP1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 12:15 - FlexBus Signal Group 5 Multiplex control"]
    #[inline]
    pub fn group5(&mut self) -> _GROUP5W {
        _GROUP5W { w: self }
    }
    #[doc = "Bits 16:19 - FlexBus Signal Group 4 Multiplex control"]
    #[inline]
    pub fn group4(&mut self) -> _GROUP4W {
        _GROUP4W { w: self }
    }
    #[doc = "Bits 20:23 - FlexBus Signal Group 3 Multiplex control"]
    #[inline]
    pub fn group3(&mut self) -> _GROUP3W {
        _GROUP3W { w: self }
    }
    #[doc = "Bits 24:27 - FlexBus Signal Group 2 Multiplex control"]
    #[inline]
    pub fn group2(&mut self) -> _GROUP2W {
        _GROUP2W { w: self }
    }
    #[doc = "Bits 28:31 - FlexBus Signal Group 1 Multiplex control"]
    #[inline]
    pub fn group1(&mut self) -> _GROUP1W {
        _GROUP1W { w: self }
    }
}
