use std::sync::{Arc, Mutex};

use crate::model::components::{
    animal_component::AnimalComponent,
    bird_component::BirdComponent,
    dragon_component::DragonComponent,
    egglaying_animal_component::{EgglayingAnimalComponent, INIT_EGGS},
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
                dragon: self.dragon.clone(),
                bird: self.bird.clone(),
                lizard: self.lizard.clone(),
                egg_laying_animal,
                animal: self.animal.clone(),
                obj: ObjComponent {
                    obj_id: ObjComponent::new_id(),
                    parent_id: Some(self.obj.obj_id.clone()),
                    obj_type: ObjType::Dragon,
                },
                shared_food: Arc::clone(&self.shared_food),
            })
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

pub fn dragon_entity_main() {
    let mut dragon = DragonEntity::new(
        DragonComponent {
            etanol_liters: 1000,
        },
        BirdComponent {},
        LizardComponent {},
        EgglayingAnimalComponent { eggs: INIT_EGGS },
        AnimalComponent { calories: 10 },
        ObjComponent {
            obj_id: ObjComponent::new_id(),
            parent_id: None,
            obj_type: ObjType::Dragon,
        },
        Arc::new(Mutex::new(FoodComponent {
            food_capacity: 100000,
        })),
    );
    dragon.fire();
    dragon.eat(50);
    dragon.peep();
    dragon.crawl();
    println!("\r\nDragon: {:?}", dragon);
    dragons_only(&mut dragon);
    if let Some(mut new_dragon) = dragon.try_reproduce() {
        new_dragon.eat(50);
        println!("\r\nChild dragon: {:?}", new_dragon);
        dragons_only(&mut new_dragon);
        if let Some(mut new_new_dragon) = new_dragon.try_reproduce() {
            new_new_dragon.eat(50);
            dragons_only(&mut new_dragon);
            println!("\r\nChild dragon: {:?}", new_dragon);
            if let Some(mut new_new_new_dragon) = new_new_dragon.try_reproduce() {
                new_new_dragon.eat(50);
                dragons_only(&mut new_new_new_dragon);
                println!("\r\nGrand grand child dragon: {:?}", new_new_new_dragon);
                if let Some(mut new_new_new_new_dragon) = new_new_new_dragon.try_reproduce() {
                    new_new_new_new_dragon.eat(50);
                    dragons_only(&mut new_new_new_new_dragon);
                    println!(
                        "\r\nGrand grand grand child dragon: {:?}",
                        new_new_new_new_dragon
                    );
                } else {
                    println!("\r\nFail reproduce dragon {:?}", new_new_new_dragon);
                }
            } else {
                println!("\r\nFail reproduce dragon {:?}", new_new_dragon);
            }
        } else {
            println!("\r\nFail reproduce dragon {:?}", new_dragon);
        }
    } else {
        println!("\r\nFail reproduce dragon {:?}", dragon);
    }
}

fn dragons_only(dragon: &mut impl DragonTrait) {
    dragon.fire();
}
