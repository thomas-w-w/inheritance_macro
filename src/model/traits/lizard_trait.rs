use crate::model::traits::egglaying_animal_trait::EgglayingAnimalTrait;

pub(crate) trait LizardTrait: EgglayingAnimalTrait {
    fn crawl(&self);
}
