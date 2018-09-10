#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBCMD {
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
pub struct RSR {
    bits: bool,
}
impl RSR {
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
pub struct RSTR {
    bits: bool,
}
impl RSTR {
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
#[doc = "Possible values of the field `FS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FSR {
    #[doc = "When FS2 = 0, the size is 1024 elements (4096 bytes). When FS2 = 1, the size is 64 elements (256 bytes)."]
    _00,
    #[doc = "When FS2 = 0, the size is 512 elements (2048 bytes). When FS2 = 1, the size is 32 elements (128 bytes)."]
    _01,
    #[doc = "When FS2 = 0, the size is 256 elements (1024 bytes). When FS2 = 1, the size is 16 elements (64 bytes)."]
    _10,
    #[doc = "When FS2 = 0, the size is 128 elements (512 bytes). When FS2 = 1, the size is 8 elements (32 bytes)."]
    _11,
}
impl FSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FSR::_00 => 0,
            FSR::_01 => 1,
            FSR::_10 => 2,
            FSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FSR {
        match value {
            0 => FSR::_00,
            1 => FSR::_01,
            2 => FSR::_10,
            3 => FSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == FSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == FSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == FSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == FSR::_11
    }
}
#[doc = "Possible values of the field `PSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSER {
    #[doc = "Do not process periodic schedule."]
    _0,
    #[doc = "Use the PERIODICLISTBASE register to access the periodic schedule."]
    _1,
}
impl PSER {
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
            PSER::_0 => false,
            PSER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PSER {
        match value {
            false => PSER::_0,
            true => PSER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PSER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PSER::_1
    }
}
#[doc = "Possible values of the field `ASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASER {
    #[doc = "Do not process asynchronous schedule."]
    _0,
    #[doc = "Use the ASYNCLISTADDR register to access asynchronous schedule."]
    _1,
}
impl ASER {
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
            ASER::_0 => false,
            ASER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASER {
        match value {
            false => ASER::_0,
            true => ASER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ASER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ASER::_1
    }
}
#[doc = r" Value of the field"]
pub struct IAAR {
    bits: bool,
}
impl IAAR {
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
pub struct ASPR {
    bits: u8,
}
impl ASPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `ASPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASPER {
    #[doc = "Park mode disabled"]
    _0,
    #[doc = "Park mode enabled"]
    _1,
}
impl ASPER {
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
            ASPER::_0 => false,
            ASPER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ASPER {
        match value {
            false => ASPER::_0,
            true => ASPER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ASPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ASPER::_1
    }
}
#[doc = r" Value of the field"]
pub struct SUTWR {
    bits: bool,
}
impl SUTWR {
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
pub struct ATDTWR {
    bits: bool,
}
impl ATDTWR {
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
pub struct FS2R {
    bits: bool,
}
impl FS2R {
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
#[doc = "Possible values of the field `ITC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITCR {
    #[doc = "Immediate (no threshold)"]
    _0,
    #[doc = "1 microframe"]
    _1,
    #[doc = "2 microframes"]
    _10,
    #[doc = "4 microframes"]
    _100,
    #[doc = "8 microframes"]
    _1000,
    #[doc = "16 microframes"]
    _10000,
    #[doc = "32 microframes"]
    _100000,
    #[doc = "64 microframes"]
    _1000000,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ITCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ITCR::_0 => 0,
            ITCR::_1 => 1,
            ITCR::_10 => 2,
            ITCR::_100 => 4,
            ITCR::_1000 => 8,
            ITCR::_10000 => 16,
            ITCR::_100000 => 32,
            ITCR::_1000000 => 64,
            ITCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ITCR {
        match value {
            0 => ITCR::_0,
            1 => ITCR::_1,
            2 => ITCR::_10,
            4 => ITCR::_100,
            8 => ITCR::_1000,
            16 => ITCR::_10000,
            32 => ITCR::_100000,
            64 => ITCR::_1000000,
            i => ITCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ITCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ITCR::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == ITCR::_10
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == ITCR::_100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == ITCR::_1000
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline]
    pub fn is_10000(&self) -> bool {
        *self == ITCR::_10000
    }
    #[doc = "Checks if the value of the field is `_100000`"]
    #[inline]
    pub fn is_100000(&self) -> bool {
        *self == ITCR::_100000
    }
    #[doc = "Checks if the value of the field is `_1000000`"]
    #[inline]
    pub fn is_1000000(&self) -> bool {
        *self == ITCR::_1000000
    }
}
#[doc = r" Proxy"]
pub struct _RSW<'a> {
    w: &'a mut W,
}
impl<'a> _RSW<'a> {
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
#[doc = r" Proxy"]
pub struct _RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FS`"]
pub enum FSW {
    #[doc = "When FS2 = 0, the size is 1024 elements (4096 bytes). When FS2 = 1, the size is 64 elements (256 bytes)."]
    _00,
    #[doc = "When FS2 = 0, the size is 512 elements (2048 bytes). When FS2 = 1, the size is 32 elements (128 bytes)."]
    _01,
    #[doc = "When FS2 = 0, the size is 256 elements (1024 bytes). When FS2 = 1, the size is 16 elements (64 bytes)."]
    _10,
    #[doc = "When FS2 = 0, the size is 128 elements (512 bytes). When FS2 = 1, the size is 8 elements (32 bytes)."]
    _11,
}
impl FSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FSW::_00 => 0,
            FSW::_01 => 1,
            FSW::_10 => 2,
            FSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FSW<'a> {
    w: &'a mut W,
}
impl<'a> _FSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "When FS2 = 0, the size is 1024 elements (4096 bytes). When FS2 = 1, the size is 64 elements (256 bytes)."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(FSW::_00)
    }
    #[doc = "When FS2 = 0, the size is 512 elements (2048 bytes). When FS2 = 1, the size is 32 elements (128 bytes)."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(FSW::_01)
    }
    #[doc = "When FS2 = 0, the size is 256 elements (1024 bytes). When FS2 = 1, the size is 16 elements (64 bytes)."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(FSW::_10)
    }
    #[doc = "When FS2 = 0, the size is 128 elements (512 bytes). When FS2 = 1, the size is 8 elements (32 bytes)."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(FSW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSE`"]
pub enum PSEW {
    #[doc = "Do not process periodic schedule."]
    _0,
    #[doc = "Use the PERIODICLISTBASE register to access the periodic schedule."]
    _1,
}
impl PSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PSEW::_0 => false,
            PSEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSEW<'a> {
    w: &'a mut W,
}
impl<'a> _PSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not process periodic schedule."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSEW::_0)
    }
    #[doc = "Use the PERIODICLISTBASE register to access the periodic schedule."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSEW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASE`"]
pub enum ASEW {
    #[doc = "Do not process asynchronous schedule."]
    _0,
    #[doc = "Use the ASYNCLISTADDR register to access asynchronous schedule."]
    _1,
}
impl ASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASEW::_0 => false,
            ASEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASEW<'a> {
    w: &'a mut W,
}
impl<'a> _ASEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not process asynchronous schedule."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASEW::_0)
    }
    #[doc = "Use the ASYNCLISTADDR register to access asynchronous schedule."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASEW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IAAW<'a> {
    w: &'a mut W,
}
impl<'a> _IAAW<'a> {
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ASPW<'a> {
    w: &'a mut W,
}
impl<'a> _ASPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ASPE`"]
pub enum ASPEW {
    #[doc = "Park mode disabled"]
    _0,
    #[doc = "Park mode enabled"]
    _1,
}
impl ASPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ASPEW::_0 => false,
            ASPEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ASPEW<'a> {
    w: &'a mut W,
}
impl<'a> _ASPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ASPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Park mode disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ASPEW::_0)
    }
    #[doc = "Park mode enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ASPEW::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SUTWW<'a> {
    w: &'a mut W,
}
impl<'a> _SUTWW<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ATDTWW<'a> {
    w: &'a mut W,
}
impl<'a> _ATDTWW<'a> {
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FS2W<'a> {
    w: &'a mut W,
}
impl<'a> _FS2W<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ITC`"]
pub enum ITCW {
    #[doc = "Immediate (no threshold)"]
    _0,
    #[doc = "1 microframe"]
    _1,
    #[doc = "2 microframes"]
    _10,
    #[doc = "4 microframes"]
    _100,
    #[doc = "8 microframes"]
    _1000,
    #[doc = "16 microframes"]
    _10000,
    #[doc = "32 microframes"]
    _100000,
    #[doc = "64 microframes"]
    _1000000,
}
impl ITCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ITCW::_0 => 0,
            ITCW::_1 => 1,
            ITCW::_10 => 2,
            ITCW::_100 => 4,
            ITCW::_1000 => 8,
            ITCW::_10000 => 16,
            ITCW::_100000 => 32,
            ITCW::_1000000 => 64,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITCW<'a> {
    w: &'a mut W,
}
impl<'a> _ITCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate (no threshold)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITCW::_0)
    }
    #[doc = "1 microframe"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITCW::_1)
    }
    #[doc = "2 microframes"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(ITCW::_10)
    }
    #[doc = "4 microframes"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(ITCW::_100)
    }
    #[doc = "8 microframes"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ITCW::_1000)
    }
    #[doc = "16 microframes"]
    #[inline]
    pub fn _10000(self) -> &'a mut W {
        self.variant(ITCW::_10000)
    }
    #[doc = "32 microframes"]
    #[inline]
    pub fn _100000(self) -> &'a mut W {
        self.variant(ITCW::_100000)
    }
    #[doc = "64 microframes"]
    #[inline]
    pub fn _1000000(self) -> &'a mut W {
        self.variant(ITCW::_1000000)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Run/Stop"]
    #[inline]
    pub fn rs(&self) -> RSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSR { bits }
    }
    #[doc = "Bit 1 - Controller Reset"]
    #[inline]
    pub fn rst(&self) -> RSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RSTR { bits }
    }
    #[doc = "Bits 2:3 - Frame list Size"]
    #[inline]
    pub fn fs(&self) -> FSR {
        FSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Periodic Schedule Enable"]
    #[inline]
    pub fn pse(&self) -> PSER {
        PSER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable"]
    #[inline]
    pub fn ase(&self) -> ASER {
        ASER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Interrupt on Async Advance doorbell"]
    #[inline]
    pub fn iaa(&self) -> IAAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IAAR { bits }
    }
    #[doc = "Bits 8:9 - Asynchronous Schedule Park mode count"]
    #[inline]
    pub fn asp(&self) -> ASPR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ASPR { bits }
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park mode Enable"]
    #[inline]
    pub fn aspe(&self) -> ASPER {
        ASPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Setup TripWire"]
    #[inline]
    pub fn sutw(&self) -> SUTWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SUTWR { bits }
    }
    #[doc = "Bit 14 - Add dTD TripWire"]
    #[inline]
    pub fn atdtw(&self) -> ATDTWR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ATDTWR { bits }
    }
    #[doc = "Bit 15 - Frame list Size 2"]
    #[inline]
    pub fn fs2(&self) -> FS2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FS2R { bits }
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control"]
    #[inline]
    pub fn itc(&self) -> ITCR {
        ITCR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 524288 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Run/Stop"]
    #[inline]
    pub fn rs(&mut self) -> _RSW {
        _RSW { w: self }
    }
    #[doc = "Bit 1 - Controller Reset"]
    #[inline]
    pub fn rst(&mut self) -> _RSTW {
        _RSTW { w: self }
    }
    #[doc = "Bits 2:3 - Frame list Size"]
    #[inline]
    pub fn fs(&mut self) -> _FSW {
        _FSW { w: self }
    }
    #[doc = "Bit 4 - Periodic Schedule Enable"]
    #[inline]
    pub fn pse(&mut self) -> _PSEW {
        _PSEW { w: self }
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable"]
    #[inline]
    pub fn ase(&mut self) -> _ASEW {
        _ASEW { w: self }
    }
    #[doc = "Bit 6 - Interrupt on Async Advance doorbell"]
    #[inline]
    pub fn iaa(&mut self) -> _IAAW {
        _IAAW { w: self }
    }
    #[doc = "Bits 8:9 - Asynchronous Schedule Park mode count"]
    #[inline]
    pub fn asp(&mut self) -> _ASPW {
        _ASPW { w: self }
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park mode Enable"]
    #[inline]
    pub fn aspe(&mut self) -> _ASPEW {
        _ASPEW { w: self }
    }
    #[doc = "Bit 13 - Setup TripWire"]
    #[inline]
    pub fn sutw(&mut self) -> _SUTWW {
        _SUTWW { w: self }
    }
    #[doc = "Bit 14 - Add dTD TripWire"]
    #[inline]
    pub fn atdtw(&mut self) -> _ATDTWW {
        _ATDTWW { w: self }
    }
    #[doc = "Bit 15 - Frame list Size 2"]
    #[inline]
    pub fn fs2(&mut self) -> _FS2W {
        _FS2W { w: self }
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control"]
    #[inline]
    pub fn itc(&mut self) -> _ITCW {
        _ITCW { w: self }
    }
}
