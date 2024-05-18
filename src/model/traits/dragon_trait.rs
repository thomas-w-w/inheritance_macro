use crate::model::traits::{bird_trait::BirdTrait, lizard_trait::LizardTrait};

pub(crate) trait DragonTrait: BirdTrait + LizardTrait {
    fn fire(&self);
}
