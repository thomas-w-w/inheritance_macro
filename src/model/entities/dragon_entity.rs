use std::sync::{Arc, Mutex};

use crate::model::components::{
    animal_component::AnimalComponent,
    bird_component::BirdComponent,
    dragon_component::DragonComponent,
    egglaying_animal_component::EgglayingAnimalComponent,
    food_component::FoodComponent,
    lizard_component::LizardComponent,
    obj_component::{ObjComponent, ObjType},
};
use crate::model::traits::{
    animal_trait::AnimalTrait, bird_trait::BirdTrait, dragon_trait::DragonTrait,
    egglaying_animal_trait::EgglayingAnimalTrait, lizard_trait::LizardTrait, obj_trait::ObjTrait,
};

#[derive(Debug)]
pub(crate) struct DragonEntity {
    dragon: DragonComponent,
    bird: BirdComponent,
    lizard: LizardComponent,
    egg_laying_animal: EgglayingAnimalComponent,
    animal: AnimalComponent,
    obj: ObjComponent,
    shared_food: Arc<Mutex<FoodComponent>>,
}

impl DragonEntity {
    pub fn new(
        dragon: DragonComponent,
        bird: BirdComponent,
        lizard: LizardComponent,
        egg_laying_animal: EgglayingAnimalComponent,
        animal: AnimalComponent,
        obj: ObjComponent,
        shared_food: Arc<Mutex<FoodComponent>>,
    ) -> Self {
        Self {
            dragon,
            bird,
            lizard,
            egg_laying_animal,
            animal,
            obj,
            shared_food,
        }
    }
}

impl ObjTrait for DragonEntity {}

impl AnimalTrait for DragonEntity {
    type Offspring = DragonEntity;
    fn eat(&mut self, calories: u32) -> bool {
        self.animal.eat(Arc::clone(&self.shared_food), calories)
    }

    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        self.egg_laying_animal
            .try_reproduce(&mut self.animal)
            .map(|egg_laying_animal| Self::Offspring {
                dragon: DragonComponent {
                    etanol_liters: self.dragon.etanol_liters,
                },
                bird: self.bird.clone(),
                lizard: self.lizard.clone(),
                egg_laying_animal,
                animal: AnimalComponent {
                    calories: self.animal.calories,
                    given_name: format!("{} child", self.animal.given_name),
                },
                obj: ObjComponent {
                    obj_id: ObjComponent::new_id(),
                    parent_id: Some(self.obj.obj_id.clone()),
                    obj_type: ObjType::Dragon,
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

impl EgglayingAnimalTrait for DragonEntity {}

impl BirdTrait for DragonEntity {
    fn peep(&self) {
        self.bird.peep()
    }
}

impl LizardTrait for DragonEntity {
    fn crawl(&self) {
        self.lizard.crawl();
    }
}

impl DragonTrait for DragonEntity {
    fn fire(&mut self) -> bool {
        self.dragon
            .fire(&mut self.animal, Arc::clone(&self.shared_food))
    }
}
