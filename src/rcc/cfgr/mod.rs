#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFGR {
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
#[doc = r" Value of the field"]
pub struct MCO2R {
    bits: u8,
}
impl MCO2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MCO2PRER {
    bits: u8,
}
impl MCO2PRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MCO1PRER {
    bits: u8,
}
impl MCO1PRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct I2SSRCR {
    bits: bool,
}
impl I2SSRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct MCO1R {
    bits: u8,
}
impl MCO1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RTCPRER {
    bits: u8,
}
impl RTCPRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PPRE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPRE2R {
    #[doc = "HCLK not divided"]
    DIV1,
    #[doc = "HCLK divided by 2"]
    DIV2,
    #[doc = "HCLK divided by 4"]
    DIV4,
    #[doc = "HCLK divided by 8"]
    DIV8,
    #[doc = "HCLK divided by 16"]
    DIV16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PPRE2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PPRE2R::DIV1 => 0,
            PPRE2R::DIV2 => 4,
            PPRE2R::DIV4 => 5,
            PPRE2R::DIV8 => 6,
            PPRE2R::DIV16 => 7,
            PPRE2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PPRE2R {
        match value {
            0 => PPRE2R::DIV1,
            4 => PPRE2R::DIV2,
            5 => PPRE2R::DIV4,
            6 => PPRE2R::DIV8,
            7 => PPRE2R::DIV16,
            i => PPRE2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PPRE2R::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PPRE2R::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PPRE2R::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PPRE2R::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PPRE2R::DIV16
    }
}
#[doc = "Possible values of the field `PPRE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPRE1R {
    #[doc = "HCLK not divided"]
    DIV1,
    #[doc = "HCLK divided by 2"]
    DIV2,
    #[doc = "HCLK divided by 4"]
    DIV4,
    #[doc = "HCLK divided by 8"]
    DIV8,
    #[doc = "HCLK divided by 16"]
    DIV16,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PPRE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PPRE1R::DIV1 => 0,
            PPRE1R::DIV2 => 4,
            PPRE1R::DIV4 => 5,
            PPRE1R::DIV8 => 6,
            PPRE1R::DIV16 => 7,
            PPRE1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PPRE1R {
        match value {
            0 => PPRE1R::DIV1,
            4 => PPRE1R::DIV2,
            5 => PPRE1R::DIV4,
            6 => PPRE1R::DIV8,
            7 => PPRE1R::DIV16,
            i => PPRE1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PPRE1R::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PPRE1R::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PPRE1R::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PPRE1R::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PPRE1R::DIV16
    }
}
#[doc = "Possible values of the field `HPRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HPRER {
    #[doc = "SYSCLK not divided"]
    DIV1,
    #[doc = "SYSCLK divided by 2"]
    DIV2,
    #[doc = "SYSCLK divided by 4"]
    DIV4,
    #[doc = "SYSCLK divided by 8"]
    DIV8,
    #[doc = "SYSCLK divided by 16"]
    DIV16,
    #[doc = "SYSCLK divided by 64"]
    DIV64,
    #[doc = "SYSCLK divided by 128"]
    DIV128,
    #[doc = "SYSCLK divided by 256"]
    DIV256,
    #[doc = "SYSCLK divided by 512"]
    DIV512,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HPRER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HPRER::DIV1 => 0,
            HPRER::DIV2 => 8,
            HPRER::DIV4 => 9,
            HPRER::DIV8 => 10,
            HPRER::DIV16 => 11,
            HPRER::DIV64 => 12,
            HPRER::DIV128 => 13,
            HPRER::DIV256 => 14,
            HPRER::DIV512 => 15,
            HPRER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HPRER {
        match value {
            0 => HPRER::DIV1,
            8 => HPRER::DIV2,
            9 => HPRER::DIV4,
            10 => HPRER::DIV8,
            11 => HPRER::DIV16,
            12 => HPRER::DIV64,
            13 => HPRER::DIV128,
            14 => HPRER::DIV256,
            15 => HPRER::DIV512,
            i => HPRER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == HPRER::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == HPRER::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == HPRER::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == HPRER::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == HPRER::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == HPRER::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline]
    pub fn is_div128(&self) -> bool {
        *self == HPRER::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == HPRER::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline]
    pub fn is_div512(&self) -> bool {
        *self == HPRER::DIV512
    }
}
#[doc = "Possible values of the field `SWS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSR {
    #[doc = " HSI selected as system clock"]
    HSI,
    #[doc = " HSE selected as system clock"]
    HSE,
    #[doc = "PLL selected as system clock"]
    PLL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SWSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SWSR::HSI => 0,
            SWSR::HSE => 1,
            SWSR::PLL => 2,
            SWSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SWSR {
        match value {
            0 => SWSR::HSI,
            1 => SWSR::HSE,
            2 => SWSR::PLL,
            i => SWSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline]
    pub fn is_hsi(&self) -> bool {
        *self == SWSR::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline]
    pub fn is_hse(&self) -> bool {
        *self == SWSR::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline]
    pub fn is_pll(&self) -> bool {
        *self == SWSR::PLL
    }
}
#[doc = "Possible values of the field `SW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR {
    #[doc = " HSI selected as system clock"]
    HSI,
    #[doc = " HSE selected as system clock"]
    HSE,
    #[doc = "PLL selected as system clock"]
    PLL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SWR::HSI => 0,
            SWR::HSE => 1,
            SWR::PLL => 2,
            SWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SWR {
        match value {
            0 => SWR::HSI,
            1 => SWR::HSE,
            2 => SWR::PLL,
            i => SWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline]
    pub fn is_hsi(&self) -> bool {
        *self == SWR::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline]
    pub fn is_hse(&self) -> bool {
        *self == SWR::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline]
    pub fn is_pll(&self) -> bool {
        *self == SWR::PLL
    }
}
#[doc = r" Proxy"]
pub struct _MCO2W<'a> {
    w: &'a mut W,
}
impl<'a> _MCO2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCO2PREW<'a> {
    w: &'a mut W,
}
impl<'a> _MCO2PREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCO1PREW<'a> {
    w: &'a mut W,
}
impl<'a> _MCO1PREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _I2SSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _I2SSRCW<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MCO1W<'a> {
    w: &'a mut W,
}
impl<'a> _MCO1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RTCPREW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCPREW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PPRE2`"]
pub enum PPRE2W {
    #[doc = "HCLK not divided"]
    DIV1,
    #[doc = "HCLK divided by 2"]
    DIV2,
    #[doc = "HCLK divided by 4"]
    DIV4,
    #[doc = "HCLK divided by 8"]
    DIV8,
    #[doc = "HCLK divided by 16"]
    DIV16,
}
impl PPRE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PPRE2W::DIV1 => 0,
            PPRE2W::DIV2 => 4,
            PPRE2W::DIV4 => 5,
            PPRE2W::DIV8 => 6,
            PPRE2W::DIV16 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPRE2W<'a> {
    w: &'a mut W,
}
impl<'a> _PPRE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPRE2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "HCLK not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE2W::DIV1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE2W::DIV2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE2W::DIV4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE2W::DIV8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE2W::DIV16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PPRE1`"]
pub enum PPRE1W {
    #[doc = "HCLK not divided"]
    DIV1,
    #[doc = "HCLK divided by 2"]
    DIV2,
    #[doc = "HCLK divided by 4"]
    DIV4,
    #[doc = "HCLK divided by 8"]
    DIV8,
    #[doc = "HCLK divided by 16"]
    DIV16,
}
impl PPRE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PPRE1W::DIV1 => 0,
            PPRE1W::DIV2 => 4,
            PPRE1W::DIV4 => 5,
            PPRE1W::DIV8 => 6,
            PPRE1W::DIV16 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPRE1W<'a> {
    w: &'a mut W,
}
impl<'a> _PPRE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPRE1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "HCLK not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE1W::DIV1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE1W::DIV2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE1W::DIV4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE1W::DIV8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE1W::DIV16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `HPRE`"]
pub enum HPREW {
    #[doc = "SYSCLK not divided"]
    DIV1,
    #[doc = "SYSCLK divided by 2"]
    DIV2,
    #[doc = "SYSCLK divided by 4"]
    DIV4,
    #[doc = "SYSCLK divided by 8"]
    DIV8,
    #[doc = "SYSCLK divided by 16"]
    DIV16,
    #[doc = "SYSCLK divided by 64"]
    DIV64,
    #[doc = "SYSCLK divided by 128"]
    DIV128,
    #[doc = "SYSCLK divided by 256"]
    DIV256,
    #[doc = "SYSCLK divided by 512"]
    DIV512,
}
impl HPREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HPREW::DIV1 => 0,
            HPREW::DIV2 => 8,
            HPREW::DIV4 => 9,
            HPREW::DIV8 => 10,
            HPREW::DIV16 => 11,
            HPREW::DIV64 => 12,
            HPREW::DIV128 => 13,
            HPREW::DIV256 => 14,
            HPREW::DIV512 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HPREW<'a> {
    w: &'a mut W,
}
impl<'a> _HPREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HPREW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SYSCLK not divided"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPREW::DIV1)
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPREW::DIV2)
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPREW::DIV4)
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPREW::DIV8)
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPREW::DIV16)
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPREW::DIV64)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPREW::DIV128)
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPREW::DIV256)
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPREW::DIV512)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SW`"]
pub enum SWW {
    #[doc = " HSI selected as system clock"]
    HSI,
    #[doc = " HSE selected as system clock"]
    HSE,
    #[doc = "PLL selected as system clock"]
    PLL,
}
impl SWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SWW::HSI => 0,
            SWW::HSE => 1,
            SWW::PLL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWW<'a> {
    w: &'a mut W,
}
impl<'a> _SWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "HSI selected as system clock"]
    #[inline]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SWW::HSI)
    }
    #[doc = "HSE selected as system clock"]
    #[inline]
    pub fn hse(self) -> &'a mut W {
        self.variant(SWW::HSE)
    }
    #[doc = "PLL selected as system clock"]
    #[inline]
    pub fn pll(self) -> &'a mut W {
        self.variant(SWW::PLL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 30:31 - Microcontroller clock output 2"]
    #[inline]
    pub fn mco2(&self) -> MCO2R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MCO2R { bits }
    }
    #[doc = "Bits 27:29 - MCO2 prescaler"]
    #[inline]
    pub fn mco2pre(&self) -> MCO2PRER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MCO2PRER { bits }
    }
    #[doc = "Bits 24:26 - MCO1 prescaler"]
    #[inline]
    pub fn mco1pre(&self) -> MCO1PRER {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MCO1PRER { bits }
    }
    #[doc = "Bit 23 - I2S clock selection"]
    #[inline]
    pub fn i2ssrc(&self) -> I2SSRCR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        I2SSRCR { bits }
    }
    #[doc = "Bits 21:22 - Microcontroller clock output 1"]
    #[inline]
    pub fn mco1(&self) -> MCO1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MCO1R { bits }
    }
    #[doc = "Bits 16:20 - HSE division factor for RTC clock"]
    #[inline]
    pub fn rtcpre(&self) -> RTCPRER {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RTCPRER { bits }
    }
    #[doc = "Bits 13:15 - APB high-speed prescaler(APB2)"]
    #[inline]
    pub fn ppre2(&self) -> PPRE2R {
        PPRE2R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:12 - APB Low speed prescaler(APB1)"]
    #[inline]
    pub fn ppre1(&self) -> PPRE1R {
        PPRE1R::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline]
    pub fn hpre(&self) -> HPRER {
        HPRER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - System clock switch status"]
    #[inline]
    pub fn sws(&self) -> SWSR {
        SWSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline]
    pub fn sw(&self) -> SWR {
        SWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
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
    #[doc = "Bits 30:31 - Microcontroller clock output 2"]
    #[inline]
    pub fn mco2(&mut self) -> _MCO2W {
        _MCO2W { w: self }
    }
    #[doc = "Bits 27:29 - MCO2 prescaler"]
    #[inline]
    pub fn mco2pre(&mut self) -> _MCO2PREW {
        _MCO2PREW { w: self }
    }
    #[doc = "Bits 24:26 - MCO1 prescaler"]
    #[inline]
    pub fn mco1pre(&mut self) -> _MCO1PREW {
        _MCO1PREW { w: self }
    }
    #[doc = "Bit 23 - I2S clock selection"]
    #[inline]
    pub fn i2ssrc(&mut self) -> _I2SSRCW {
        _I2SSRCW { w: self }
    }
    #[doc = "Bits 21:22 - Microcontroller clock output 1"]
    #[inline]
    pub fn mco1(&mut self) -> _MCO1W {
        _MCO1W { w: self }
    }
    #[doc = "Bits 16:20 - HSE division factor for RTC clock"]
    #[inline]
    pub fn rtcpre(&mut self) -> _RTCPREW {
        _RTCPREW { w: self }
    }
    #[doc = "Bits 13:15 - APB high-speed prescaler(APB2)"]
    #[inline]
    pub fn ppre2(&mut self) -> _PPRE2W {
        _PPRE2W { w: self }
    }
    #[doc = "Bits 10:12 - APB Low speed prescaler(APB1)"]
    #[inline]
    pub fn ppre1(&mut self) -> _PPRE1W {
        _PPRE1W { w: self }
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline]
    pub fn hpre(&mut self) -> _HPREW {
        _HPREW { w: self }
    }
    #[doc = "Bits 0:1 - System clock switch"]
    #[inline]
    pub fn sw(&mut self) -> _SWW {
        _SWW { w: self }
    }
}
