use std::sync::{Arc, Mutex};

use crate::model::components::{
    animal_component::AnimalComponent,
    bird_component::BirdComponent,
    egglaying_animal_component::EgglayingAnimalComponent,
    food_component::FoodComponent,
    obj_component::{ObjComponent, ObjType},
};
use crate::model::traits::{
    animal_trait::AnimalTrait, bird_trait::BirdTrait, egglaying_animal_trait::EgglayingAnimalTrait,
    obj_trait::ObjTrait,
};

#[derive(Debug)]
pub(crate) struct BirdEntity {
    bird: BirdComponent,
    animal: AnimalComponent,
    egg_laying_animal: EgglayingAnimalComponent,
    obj: ObjComponent,
    shared_food: Arc<Mutex<FoodComponent>>,
}

impl BirdEntity {
    pub fn new(
        bird: BirdComponent,
        animal: AnimalComponent,
        egg_laying_animal: EgglayingAnimalComponent,
        obj: ObjComponent,
        shared_food: Arc<Mutex<FoodComponent>>,
    ) -> Self {
        Self {
            bird,
            animal,
            egg_laying_animal,
            obj,
            shared_food,
        }
    }
}

impl ObjTrait for BirdEntity {}

impl EgglayingAnimalTrait for BirdEntity {}

impl AnimalTrait for BirdEntity {
    type Offspring = BirdEntity;
    fn eat(&mut self, calories: u32) -> bool {
        self.animal.eat(Arc::clone(&self.shared_food), calories)
    }

    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        self.egg_laying_animal
            .try_reproduce(&mut self.animal)
            .map(|egg_laying_animal| Self::Offspring {
                bird: self.bird.clone(),
                egg_laying_animal,
                animal: AnimalComponent {
                    calories: self.animal.calories,
                    given_name: format!("{} child", self.animal.given_name),
                },
                obj: ObjComponent {
                    obj_id: ObjComponent::new_id(),
                    parent_id: Some(self.obj.obj_id.clone()),
                    obj_type: ObjType::Bird,
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

impl BirdTrait for BirdEntity {
    fn peep(&self) {
        self.bird.peep()
    }
}
