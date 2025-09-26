#[doc = "Register `DATA31` reader"]
pub type R = crate::R<DATA31_SPEC>;
#[doc = "Register `DATA31` writer"]
pub type W = crate::W<DATA31_SPEC>;
#[doc = "Field `DATA` reader - Backup data"]
pub type DATA_R = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Backup data"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Backup data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<DATA31_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "Backup data register 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data31::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data31::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA31_SPEC;
impl crate::RegisterSpec for DATA31_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`data31::R`](R) reader structure"]
impl crate::Readable for DATA31_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data31::W`](W) writer structure"]
impl crate::Writable for DATA31_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DATA31 to value 0"]
impl crate::Resettable for DATA31_SPEC {
    const RESET_VALUE: u16 = 0;
}
