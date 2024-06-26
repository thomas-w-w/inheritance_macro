use std::sync::{Arc, Mutex};

use crate::model::components::{
    animal_component::AnimalComponent,
    egglaying_animal_component::EgglayingAnimalComponent,
    food_component::FoodComponent,
    lizard_component::LizardComponent,
    obj_component::{ObjComponent, ObjType},
};
use crate::model::traits::{
    animal_trait::AnimalTrait, egglaying_animal_trait::EgglayingAnimalTrait,
    lizard_trait::LizardTrait, obj_trait::ObjTrait,
};

#[derive(Debug, Clone)]
pub(crate) struct LizardEntity {
    lizard: LizardComponent,
    egg_laying_animal: EgglayingAnimalComponent,
    animal: AnimalComponent,
    obj: ObjComponent,
    shared_food: Arc<Mutex<FoodComponent>>,
}

impl LizardEntity {
    pub fn new(
        lizard: LizardComponent,
        egg_laying_animal: EgglayingAnimalComponent,
        animal: AnimalComponent,
        obj: ObjComponent,
        shared_food: Arc<Mutex<FoodComponent>>,
    ) -> Self {
        Self {
            lizard,
            egg_laying_animal,
            animal,
            obj,
            shared_food,
        }
    }
}

impl ObjTrait for LizardEntity {}

impl AnimalTrait for LizardEntity {
    type Offspring = LizardEntity;
    fn eat(&mut self, calories: u32) -> bool {
        self.animal.eat(Arc::clone(&self.shared_food), calories)
    }

    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        self.egg_laying_animal
            .try_reproduce(&mut self.animal)
            .map(|egg_laying_animal| Self::Offspring {
                lizard: self.lizard.clone(),
                egg_laying_animal,
                animal: AnimalComponent {
                    calories: self.animal.calories,
                    given_name: format!("{} child", self.animal.given_name),
                },
                obj: ObjComponent {
                    obj_id: ObjComponent::new_id(),
                    parent_id: Some(self.obj.obj_id.clone()),
                    obj_type: ObjType::Lizard,
                },
                shared_food: Arc::clone(&self.shared_food),
            })
    }
    fn get_given_name(&self) -> String {
        self.animal.given_name.to_owned()
    }
    fn set_given_name(&mut self, given_name: String) {
        self.animal.given_name = given_name;
    }
}

impl EgglayingAnimalTrait for LizardEntity {}

impl LizardTrait for LizardEntity {
    fn crawl(&self) {
        self.lizard.crawl()
    }
}
