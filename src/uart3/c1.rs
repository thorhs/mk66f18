#[doc = "Reader of register C1"]
pub type R = crate::R<u8, super::C1>;
#[doc = "Writer for register C1"]
pub type W = crate::W<u8, super::C1>;
#[doc = "Register C1 `reset()`'s with value 0"]
impl crate::ResetValue for super::C1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT_A {
    #[doc = "0: Even parity."]
    _0,
    #[doc = "1: Odd parity."]
    _1,
}
impl From<PT_A> for bool {
    #[inline(always)]
    fn from(variant: PT_A) -> Self {
        match variant {
            PT_A::_0 => false,
            PT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PT`"]
pub type PT_R = crate::R<bool, PT_A>;
impl PT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT_A {
        match self.bits {
            false => PT_A::_0,
            true => PT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PT_A::_1
    }
}
#[doc = "Write proxy for field `PT`"]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Even parity."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PT_A::_0)
    }
    #[doc = "Odd parity."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PT_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "0: Parity function disabled."]
    _0,
    #[doc = "1: Parity function enabled."]
    _1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        match variant {
            PE_A::_0 => false,
            PE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, PE_A>;
impl PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::_0,
            true => PE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PE_A::_1
    }
}
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Parity function disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PE_A::_0)
    }
    #[doc = "Parity function enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Idle Line Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILT_A {
    #[doc = "0: Idle character bit count starts after start bit."]
    _0,
    #[doc = "1: Idle character bit count starts after stop bit."]
    _1,
}
impl From<ILT_A> for bool {
    #[inline(always)]
    fn from(variant: ILT_A) -> Self {
        match variant {
            ILT_A::_0 => false,
            ILT_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `ILT`"]
pub type ILT_R = crate::R<bool, ILT_A>;
impl ILT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILT_A {
        match self.bits {
            false => ILT_A::_0,
            true => ILT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILT_A::_1
    }
}
#[doc = "Write proxy for field `ILT`"]
pub struct ILT_W<'a> {
    w: &'a mut W,
}
impl<'a> ILT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Idle character bit count starts after start bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILT_A::_0)
    }
    #[doc = "Idle character bit count starts after stop bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILT_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Receiver Wakeup Method Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_A {
    #[doc = "0: Idle line wakeup."]
    _0,
    #[doc = "1: Address mark wakeup."]
    _1,
}
impl From<WAKE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE_A) -> Self {
        match variant {
            WAKE_A::_0 => false,
            WAKE_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `WAKE`"]
pub type WAKE_R = crate::R<bool, WAKE_A>;
impl WAKE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKE_A {
        match self.bits {
            false => WAKE_A::_0,
            true => WAKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKE_A::_1
    }
}
#[doc = "Write proxy for field `WAKE`"]
pub struct WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Idle line wakeup."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKE_A::_0)
    }
    #[doc = "Address mark wakeup."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "9-bit or 8-bit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M_A {
    #[doc = "0: Normal-start + 8 data bits (MSB/LSB first as determined by MSBF) + stop."]
    _0,
    #[doc = "1: Use-start + 9 data bits (MSB/LSB first as determined by MSBF) + stop."]
    _1,
}
impl From<M_A> for bool {
    #[inline(always)]
    fn from(variant: M_A) -> Self {
        match variant {
            M_A::_0 => false,
            M_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `M`"]
pub type M_R = crate::R<bool, M_A>;
impl M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_A {
        match self.bits {
            false => M_A::_0,
            true => M_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M_A::_1
    }
}
#[doc = "Write proxy for field `M`"]
pub struct M_W<'a> {
    w: &'a mut W,
}
impl<'a> M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal-start + 8 data bits (MSB/LSB first as determined by MSBF) + stop."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M_A::_0)
    }
    #[doc = "Use-start + 9 data bits (MSB/LSB first as determined by MSBF) + stop."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Receiver Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSRC_A {
    #[doc = "0: Selects internal loop back mode. The receiver input is internally connected to transmitter output."]
    _0,
    #[doc = "1: Single wire UART mode where the receiver input is connected to the transmit pin input signal."]
    _1,
}
impl From<RSRC_A> for bool {
    #[inline(always)]
    fn from(variant: RSRC_A) -> Self {
        match variant {
            RSRC_A::_0 => false,
            RSRC_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `RSRC`"]
pub type RSRC_R = crate::R<bool, RSRC_A>;
impl RSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSRC_A {
        match self.bits {
            false => RSRC_A::_0,
            true => RSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSRC_A::_1
    }
}
#[doc = "Write proxy for field `RSRC`"]
pub struct RSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects internal loop back mode. The receiver input is internally connected to transmitter output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSRC_A::_0)
    }
    #[doc = "Single wire UART mode where the receiver input is connected to the transmit pin input signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSRC_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "UART Stops in Wait Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UARTSWAI_A {
    #[doc = "0: UART clock continues to run in Wait mode."]
    _0,
    #[doc = "1: UART clock freezes while CPU is in Wait mode."]
    _1,
}
impl From<UARTSWAI_A> for bool {
    #[inline(always)]
    fn from(variant: UARTSWAI_A) -> Self {
        match variant {
            UARTSWAI_A::_0 => false,
            UARTSWAI_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `UARTSWAI`"]
pub type UARTSWAI_R = crate::R<bool, UARTSWAI_A>;
impl UARTSWAI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UARTSWAI_A {
        match self.bits {
            false => UARTSWAI_A::_0,
            true => UARTSWAI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UARTSWAI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UARTSWAI_A::_1
    }
}
#[doc = "Write proxy for field `UARTSWAI`"]
pub struct UARTSWAI_W<'a> {
    w: &'a mut W,
}
impl<'a> UARTSWAI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UARTSWAI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART clock continues to run in Wait mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UARTSWAI_A::_0)
    }
    #[doc = "UART clock freezes while CPU is in Wait mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UARTSWAI_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Loop Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPS_A {
    #[doc = "0: Normal operation."]
    _0,
    #[doc = "1: Loop mode where transmitter output is internally connected to receiver input. The receiver input is determined by RSRC."]
    _1,
}
impl From<LOOPS_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPS_A) -> Self {
        match variant {
            LOOPS_A::_0 => false,
            LOOPS_A::_1 => true,
        }
    }
}
#[doc = "Reader of field `LOOPS`"]
pub type LOOPS_R = crate::R<bool, LOOPS_A>;
impl LOOPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPS_A {
        match self.bits {
            false => LOOPS_A::_0,
            true => LOOPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOOPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOOPS_A::_1
    }
}
#[doc = "Write proxy for field `LOOPS`"]
pub struct LOOPS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOOPS_A::_0)
    }
    #[doc = "Loop mode where transmitter output is internally connected to receiver input. The receiver input is determined by RSRC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOOPS_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    pub fn ilt(&self) -> ILT_R {
        ILT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 9-bit or 8-bit Mode Select"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    pub fn rsrc(&self) -> RSRC_R {
        RSRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART Stops in Wait Mode"]
    #[inline(always)]
    pub fn uartswai(&self) -> UARTSWAI_R {
        UARTSWAI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    pub fn loops(&self) -> LOOPS_R {
        LOOPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    pub fn ilt(&mut self) -> ILT_W {
        ILT_W { w: self }
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W {
        WAKE_W { w: self }
    }
    #[doc = "Bit 4 - 9-bit or 8-bit Mode Select"]
    #[inline(always)]
    pub fn m(&mut self) -> M_W {
        M_W { w: self }
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    pub fn rsrc(&mut self) -> RSRC_W {
        RSRC_W { w: self }
    }
    #[doc = "Bit 6 - UART Stops in Wait Mode"]
    #[inline(always)]
    pub fn uartswai(&mut self) -> UARTSWAI_W {
        UARTSWAI_W { w: self }
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    pub fn loops(&mut self) -> LOOPS_W {
        LOOPS_W { w: self }
    }
}
