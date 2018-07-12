#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AHB2ENR {
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
        R { bits: self.register.get() }
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
#[doc = "Possible values of the field `OTGFSEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTGFSENR {
    #[doc = "Clock disabled"]
    DISABLED,
    #[doc = "Clock enabled"]
    ENABLED,
}
impl OTGFSENR {
    #[doc = r" Returns `true` if the bit is clear(0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set(1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            OTGFSENR::DISABLED => false,
            OTGFSENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OTGFSENR {
        match value {
            false => OTGFSENR::DISABLED,
            true => OTGFSENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == OTGFSENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == OTGFSENR::ENABLED
    }
}
#[doc = "Possible values of the field `RNGEN`"]
pub type RNGENR = OTGFSENR;
#[doc = "Possible values of the field `DCMIEN`"]
pub type DCMIENR = OTGFSENR;
#[doc = "Values that can be written to the field `OTGFSEN`"]
pub enum OTGFSENW {
    #[doc = "Clock disabled"]
    DISABLED,
    #[doc = "Clock enabled"]
    ENABLED,
}
impl OTGFSENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OTGFSENW::DISABLED => false,
            OTGFSENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OTGFSENW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGFSENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OTGFSENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGFSENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGFSENW::ENABLED)
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
#[doc = "Values that can be written to the field `RNGEN`"]
pub type RNGENW = OTGFSENW;
#[doc = r" Proxy"]
pub struct _RNGENW<'a> {
    w: &'a mut W,
}
impl<'a> _RNGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RNGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGFSENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGFSENW::ENABLED)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCMIEN`"]
pub type DCMIENW = OTGFSENW;
#[doc = r" Proxy"]
pub struct _DCMIENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCMIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCMIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGFSENW::DISABLED)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGFSENW::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 7 - USB OTG FS clock enable"]
    #[inline]
    pub fn otgfsen(&self) -> OTGFSENR {
        OTGFSENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Random number generator clock enable"]
    #[inline]
    pub fn rngen(&self) -> RNGENR {
        RNGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 0 - Camera interface enable"]
    #[inline]
    pub fn dcmien(&self) -> DCMIENR {
        DCMIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 7 - USB OTG FS clock enable"]
    #[inline]
    pub fn otgfsen(&mut self) -> _OTGFSENW {
        _OTGFSENW { w: self }
    }
    #[doc = "Bit 6 - Random number generator clock enable"]
    #[inline]
    pub fn rngen(&mut self) -> _RNGENW {
        _RNGENW { w: self }
    }
    #[doc = "Bit 0 - Camera interface enable"]
    #[inline]
    pub fn dcmien(&mut self) -> _DCMIENW {
        _DCMIENW { w: self }
    }
}
