use crate::model::traits::egglaying_animal_trait::EgglayingAnimalTrait;

pub(crate) trait BirdTrait: EgglayingAnimalTrait {
    fn peep(&self);
}
