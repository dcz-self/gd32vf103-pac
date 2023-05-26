#[doc = "Register `PD` reader"]
pub struct R(crate::R<PD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD` writer"]
pub struct W(crate::W<PD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD0` reader - Interrupt pending status of line 0"]
pub type PD0_R = crate::BitReader<bool>;
#[doc = "Field `PD0` writer - Interrupt pending status of line 0"]
pub type PD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD1` reader - Interrupt pending status of line 1"]
pub type PD1_R = crate::BitReader<bool>;
#[doc = "Field `PD1` writer - Interrupt pending status of line 1"]
pub type PD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD2` reader - Interrupt pending status of line 2"]
pub type PD2_R = crate::BitReader<bool>;
#[doc = "Field `PD2` writer - Interrupt pending status of line 2"]
pub type PD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD3` reader - Interrupt pending status of line 3"]
pub type PD3_R = crate::BitReader<bool>;
#[doc = "Field `PD3` writer - Interrupt pending status of line 3"]
pub type PD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD4` reader - Interrupt pending status of line 4"]
pub type PD4_R = crate::BitReader<bool>;
#[doc = "Field `PD4` writer - Interrupt pending status of line 4"]
pub type PD4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD5` reader - Interrupt pending status of line 5"]
pub type PD5_R = crate::BitReader<bool>;
#[doc = "Field `PD5` writer - Interrupt pending status of line 5"]
pub type PD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD6` reader - Interrupt pending status of line 6"]
pub type PD6_R = crate::BitReader<bool>;
#[doc = "Field `PD6` writer - Interrupt pending status of line 6"]
pub type PD6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD7` reader - Interrupt pending status of line 7"]
pub type PD7_R = crate::BitReader<bool>;
#[doc = "Field `PD7` writer - Interrupt pending status of line 7"]
pub type PD7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD8` reader - Interrupt pending status of line 8"]
pub type PD8_R = crate::BitReader<bool>;
#[doc = "Field `PD8` writer - Interrupt pending status of line 8"]
pub type PD8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD9` reader - Interrupt pending status of line 9"]
pub type PD9_R = crate::BitReader<bool>;
#[doc = "Field `PD9` writer - Interrupt pending status of line 9"]
pub type PD9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD10` reader - Interrupt pending status of line 10"]
pub type PD10_R = crate::BitReader<bool>;
#[doc = "Field `PD10` writer - Interrupt pending status of line 10"]
pub type PD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD11` reader - Interrupt pending status of line 11"]
pub type PD11_R = crate::BitReader<bool>;
#[doc = "Field `PD11` writer - Interrupt pending status of line 11"]
pub type PD11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD12` reader - Interrupt pending status of line 12"]
pub type PD12_R = crate::BitReader<bool>;
#[doc = "Field `PD12` writer - Interrupt pending status of line 12"]
pub type PD12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD13` reader - Interrupt pending status of line 13"]
pub type PD13_R = crate::BitReader<bool>;
#[doc = "Field `PD13` writer - Interrupt pending status of line 13"]
pub type PD13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD14` reader - Interrupt pending status of line 14"]
pub type PD14_R = crate::BitReader<bool>;
#[doc = "Field `PD14` writer - Interrupt pending status of line 14"]
pub type PD14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD15` reader - Interrupt pending status of line 15"]
pub type PD15_R = crate::BitReader<bool>;
#[doc = "Field `PD15` writer - Interrupt pending status of line 15"]
pub type PD15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD16` reader - Interrupt pending status of line 16"]
pub type PD16_R = crate::BitReader<bool>;
#[doc = "Field `PD16` writer - Interrupt pending status of line 16"]
pub type PD16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD17` reader - Interrupt pending status of line 17"]
pub type PD17_R = crate::BitReader<bool>;
#[doc = "Field `PD17` writer - Interrupt pending status of line 17"]
pub type PD17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
#[doc = "Field `PD18` reader - Interrupt pending status of line 18"]
pub type PD18_R = crate::BitReader<bool>;
#[doc = "Field `PD18` writer - Interrupt pending status of line 18"]
pub type PD18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt pending status of line 0"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt pending status of line 1"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt pending status of line 2"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt pending status of line 3"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt pending status of line 4"]
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt pending status of line 5"]
    #[inline(always)]
    pub fn pd5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt pending status of line 6"]
    #[inline(always)]
    pub fn pd6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt pending status of line 7"]
    #[inline(always)]
    pub fn pd7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt pending status of line 8"]
    #[inline(always)]
    pub fn pd8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt pending status of line 9"]
    #[inline(always)]
    pub fn pd9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt pending status of line 10"]
    #[inline(always)]
    pub fn pd10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt pending status of line 11"]
    #[inline(always)]
    pub fn pd11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt pending status of line 12"]
    #[inline(always)]
    pub fn pd12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt pending status of line 13"]
    #[inline(always)]
    pub fn pd13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt pending status of line 14"]
    #[inline(always)]
    pub fn pd14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt pending status of line 15"]
    #[inline(always)]
    pub fn pd15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt pending status of line 16"]
    #[inline(always)]
    pub fn pd16(&self) -> PD16_R {
        PD16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt pending status of line 17"]
    #[inline(always)]
    pub fn pd17(&self) -> PD17_R {
        PD17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt pending status of line 18"]
    #[inline(always)]
    pub fn pd18(&self) -> PD18_R {
        PD18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt pending status of line 0"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<0> {
        PD0_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt pending status of line 1"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<1> {
        PD1_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt pending status of line 2"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<2> {
        PD2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt pending status of line 3"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<3> {
        PD3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt pending status of line 4"]
    #[inline(always)]
    #[must_use]
    pub fn pd4(&mut self) -> PD4_W<4> {
        PD4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt pending status of line 5"]
    #[inline(always)]
    #[must_use]
    pub fn pd5(&mut self) -> PD5_W<5> {
        PD5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt pending status of line 6"]
    #[inline(always)]
    #[must_use]
    pub fn pd6(&mut self) -> PD6_W<6> {
        PD6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt pending status of line 7"]
    #[inline(always)]
    #[must_use]
    pub fn pd7(&mut self) -> PD7_W<7> {
        PD7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt pending status of line 8"]
    #[inline(always)]
    #[must_use]
    pub fn pd8(&mut self) -> PD8_W<8> {
        PD8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt pending status of line 9"]
    #[inline(always)]
    #[must_use]
    pub fn pd9(&mut self) -> PD9_W<9> {
        PD9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt pending status of line 10"]
    #[inline(always)]
    #[must_use]
    pub fn pd10(&mut self) -> PD10_W<10> {
        PD10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt pending status of line 11"]
    #[inline(always)]
    #[must_use]
    pub fn pd11(&mut self) -> PD11_W<11> {
        PD11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt pending status of line 12"]
    #[inline(always)]
    #[must_use]
    pub fn pd12(&mut self) -> PD12_W<12> {
        PD12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt pending status of line 13"]
    #[inline(always)]
    #[must_use]
    pub fn pd13(&mut self) -> PD13_W<13> {
        PD13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt pending status of line 14"]
    #[inline(always)]
    #[must_use]
    pub fn pd14(&mut self) -> PD14_W<14> {
        PD14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt pending status of line 15"]
    #[inline(always)]
    #[must_use]
    pub fn pd15(&mut self) -> PD15_W<15> {
        PD15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt pending status of line 16"]
    #[inline(always)]
    #[must_use]
    pub fn pd16(&mut self) -> PD16_W<16> {
        PD16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt pending status of line 17"]
    #[inline(always)]
    #[must_use]
    pub fn pd17(&mut self) -> PD17_W<17> {
        PD17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt pending status of line 18"]
    #[inline(always)]
    #[must_use]
    pub fn pd18(&mut self) -> PD18_W<18> {
        PD18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pending register (EXTI_PD)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd](index.html) module"]
pub struct PD_SPEC;
impl crate::RegisterSpec for PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd::R](R) reader structure"]
impl crate::Readable for PD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd::W](W) writer structure"]
impl crate::Writable for PD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PD to value 0"]
impl crate::Resettable for PD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
