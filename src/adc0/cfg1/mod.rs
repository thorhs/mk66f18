#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG1 {
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
#[doc = "Possible values of the field `ADICLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADICLKR {
    #[doc = "Bus clock"]
    _00,
    #[doc = "Bus clock divided by 2(BUSCLK/2)"]
    _01,
    #[doc = "Alternate clock (ALTCLK)"]
    _10,
    #[doc = "Asynchronous clock (ADACK)"]
    _11,
}
impl ADICLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADICLKR::_00 => 0,
            ADICLKR::_01 => 1,
            ADICLKR::_10 => 2,
            ADICLKR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADICLKR {
        match value {
            0 => ADICLKR::_00,
            1 => ADICLKR::_01,
            2 => ADICLKR::_10,
            3 => ADICLKR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == ADICLKR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == ADICLKR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == ADICLKR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == ADICLKR::_11
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "When DIFF=0:It is single-ended 8-bit conversion; when DIFF=1, it is differential 9-bit conversion with 2's complement output."]
    _00,
    #[doc = "When DIFF=0:It is single-ended 12-bit conversion ; when DIFF=1, it is differential 13-bit conversion with 2's complement output."]
    _01,
    #[doc = "When DIFF=0:It is single-ended 10-bit conversion. ; when DIFF=1, it is differential 11-bit conversion with 2's complement output"]
    _10,
    #[doc = "When DIFF=0:It is single-ended 16-bit conversion..; when DIFF=1, it is differential 16-bit conversion with 2's complement output"]
    _11,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::_00 => 0,
            MODER::_01 => 1,
            MODER::_10 => 2,
            MODER::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::_00,
            1 => MODER::_01,
            2 => MODER::_10,
            3 => MODER::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == MODER::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == MODER::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == MODER::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == MODER::_11
    }
}
#[doc = "Possible values of the field `ADLSMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADLSMPR {
    #[doc = "Short sample time."]
    _0,
    #[doc = "Long sample time."]
    _1,
}
impl ADLSMPR {
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
            ADLSMPR::_0 => false,
            ADLSMPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADLSMPR {
        match value {
            false => ADLSMPR::_0,
            true => ADLSMPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADLSMPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADLSMPR::_1
    }
}
#[doc = "Possible values of the field `ADIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADIVR {
    #[doc = "The divide ratio is 1 and the clock rate is input clock."]
    _00,
    #[doc = "The divide ratio is 2 and the clock rate is (input clock)/2."]
    _01,
    #[doc = "The divide ratio is 4 and the clock rate is (input clock)/4."]
    _10,
    #[doc = "The divide ratio is 8 and the clock rate is (input clock)/8."]
    _11,
}
impl ADIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADIVR::_00 => 0,
            ADIVR::_01 => 1,
            ADIVR::_10 => 2,
            ADIVR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADIVR {
        match value {
            0 => ADIVR::_00,
            1 => ADIVR::_01,
            2 => ADIVR::_10,
            3 => ADIVR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == ADIVR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == ADIVR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == ADIVR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == ADIVR::_11
    }
}
#[doc = "Possible values of the field `ADLPC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADLPCR {
    #[doc = "Normal power configuration."]
    _0,
    #[doc = "Low-power configuration. The power is reduced at the expense of maximum clock speed."]
    _1,
}
impl ADLPCR {
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
            ADLPCR::_0 => false,
            ADLPCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADLPCR {
        match value {
            false => ADLPCR::_0,
            true => ADLPCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADLPCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADLPCR::_1
    }
}
#[doc = "Values that can be written to the field `ADICLK`"]
pub enum ADICLKW {
    #[doc = "Bus clock"]
    _00,
    #[doc = "Bus clock divided by 2(BUSCLK/2)"]
    _01,
    #[doc = "Alternate clock (ALTCLK)"]
    _10,
    #[doc = "Asynchronous clock (ADACK)"]
    _11,
}
impl ADICLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADICLKW::_00 => 0,
            ADICLKW::_01 => 1,
            ADICLKW::_10 => 2,
            ADICLKW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADICLKW<'a> {
    w: &'a mut W,
}
impl<'a> _ADICLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADICLKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bus clock"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADICLKW::_00)
    }
    #[doc = "Bus clock divided by 2(BUSCLK/2)"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADICLKW::_01)
    }
    #[doc = "Alternate clock (ALTCLK)"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADICLKW::_10)
    }
    #[doc = "Asynchronous clock (ADACK)"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADICLKW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "When DIFF=0:It is single-ended 8-bit conversion; when DIFF=1, it is differential 9-bit conversion with 2's complement output."]
    _00,
    #[doc = "When DIFF=0:It is single-ended 12-bit conversion ; when DIFF=1, it is differential 13-bit conversion with 2's complement output."]
    _01,
    #[doc = "When DIFF=0:It is single-ended 10-bit conversion. ; when DIFF=1, it is differential 11-bit conversion with 2's complement output"]
    _10,
    #[doc = "When DIFF=0:It is single-ended 16-bit conversion..; when DIFF=1, it is differential 16-bit conversion with 2's complement output"]
    _11,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::_00 => 0,
            MODEW::_01 => 1,
            MODEW::_10 => 2,
            MODEW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "When DIFF=0:It is single-ended 8-bit conversion; when DIFF=1, it is differential 9-bit conversion with 2's complement output."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(MODEW::_00)
    }
    #[doc = "When DIFF=0:It is single-ended 12-bit conversion ; when DIFF=1, it is differential 13-bit conversion with 2's complement output."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(MODEW::_01)
    }
    #[doc = "When DIFF=0:It is single-ended 10-bit conversion. ; when DIFF=1, it is differential 11-bit conversion with 2's complement output"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(MODEW::_10)
    }
    #[doc = "When DIFF=0:It is single-ended 16-bit conversion..; when DIFF=1, it is differential 16-bit conversion with 2's complement output"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(MODEW::_11)
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
#[doc = "Values that can be written to the field `ADLSMP`"]
pub enum ADLSMPW {
    #[doc = "Short sample time."]
    _0,
    #[doc = "Long sample time."]
    _1,
}
impl ADLSMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADLSMPW::_0 => false,
            ADLSMPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADLSMPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADLSMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADLSMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Short sample time."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADLSMPW::_0)
    }
    #[doc = "Long sample time."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADLSMPW::_1)
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
#[doc = "Values that can be written to the field `ADIV`"]
pub enum ADIVW {
    #[doc = "The divide ratio is 1 and the clock rate is input clock."]
    _00,
    #[doc = "The divide ratio is 2 and the clock rate is (input clock)/2."]
    _01,
    #[doc = "The divide ratio is 4 and the clock rate is (input clock)/4."]
    _10,
    #[doc = "The divide ratio is 8 and the clock rate is (input clock)/8."]
    _11,
}
impl ADIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADIVW::_00 => 0,
            ADIVW::_01 => 1,
            ADIVW::_10 => 2,
            ADIVW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADIVW<'a> {
    w: &'a mut W,
}
impl<'a> _ADIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The divide ratio is 1 and the clock rate is input clock."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADIVW::_00)
    }
    #[doc = "The divide ratio is 2 and the clock rate is (input clock)/2."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADIVW::_01)
    }
    #[doc = "The divide ratio is 4 and the clock rate is (input clock)/4."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADIVW::_10)
    }
    #[doc = "The divide ratio is 8 and the clock rate is (input clock)/8."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADIVW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADLPC`"]
pub enum ADLPCW {
    #[doc = "Normal power configuration."]
    _0,
    #[doc = "Low-power configuration. The power is reduced at the expense of maximum clock speed."]
    _1,
}
impl ADLPCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADLPCW::_0 => false,
            ADLPCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADLPCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADLPCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADLPCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal power configuration."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADLPCW::_0)
    }
    #[doc = "Low-power configuration. The power is reduced at the expense of maximum clock speed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADLPCW::_1)
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
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline]
    pub fn adiclk(&self) -> ADICLKR {
        ADICLKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Conversion mode selection"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Sample Time Configuration"]
    #[inline]
    pub fn adlsmp(&self) -> ADLSMPR {
        ADLSMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline]
    pub fn adiv(&self) -> ADIVR {
        ADIVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline]
    pub fn adlpc(&self) -> ADLPCR {
        ADLPCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline]
    pub fn adiclk(&mut self) -> _ADICLKW {
        _ADICLKW { w: self }
    }
    #[doc = "Bits 2:3 - Conversion mode selection"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 4 - Sample Time Configuration"]
    #[inline]
    pub fn adlsmp(&mut self) -> _ADLSMPW {
        _ADLSMPW { w: self }
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline]
    pub fn adiv(&mut self) -> _ADIVW {
        _ADIVW { w: self }
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline]
    pub fn adlpc(&mut self) -> _ADLPCW {
        _ADLPCW { w: self }
    }
}
