use crate::model::components::{
    animal_component::AnimalComponent,
    bird_component::BirdComponent,
    dragon_component::DragonComponent,
    egglaying_animal_component::{EgglayingAnimalComponent, INIT_EGGS},
    lizard_component::LizardComponent,
    obj_component::{ObjComponent, ObjType},
};
use crate::model::traits::{
    animal_trait::AnimalTrait, bird_trait::BirdTrait, dragon_trait::DragonTrait,
    egglaying_animal_trait::EgglayingAnimalTrait, lizard_trait::LizardTrait, obj_trait::ObjTrait,
};

#[derive(Debug)]
struct DragonEntity {
    dragon: DragonComponent,
    bird: BirdComponent,
    lizard: LizardComponent,
    egg_laying_animal: EgglayingAnimalComponent,
    animal: AnimalComponent,
    obj: ObjComponent,
}

impl DragonEntity {
    pub fn new(
        dragon: DragonComponent,
        bird: BirdComponent,
        lizard: LizardComponent,
        egg_laying_animal: EgglayingAnimalComponent,
        animal: AnimalComponent,
        obj: ObjComponent,
    ) -> Self {
        Self {
            dragon,
            bird,
            lizard,
            egg_laying_animal,
            animal,
            obj,
        }
    }
}

impl ObjTrait for DragonEntity {}

impl AnimalTrait for DragonEntity {
    type Offspring = DragonEntity;
    fn eat(&mut self, calories: u32) {
        self.animal.eat(calories)
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
    fn fire(&self) {
        self.dragon.fire();
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
    );
    dragon.fire();
    dragon.eat(50);
    dragon.peep();
    dragon.crawl();
    println!("\r\nDragon: {:?}", dragon);
    dragons_only(&dragon);
    if let Some(mut new_dragon) = dragon.try_reproduce() {
        new_dragon.eat(50);
        println!("\r\nChild dragon: {:?}", new_dragon);
        dragons_only(&new_dragon);
        if let Some(mut new_new_dragon) = new_dragon.try_reproduce() {
            new_new_dragon.eat(50);
            dragons_only(&new_dragon);
            println!("\r\nChild dragon: {:?}", new_dragon);
            if let Some(mut new_new_new_dragon) = new_new_dragon.try_reproduce() {
                new_new_dragon.eat(50);
                dragons_only(&new_new_new_dragon);
                println!("\r\nGrand grand child dragon: {:?}", new_new_new_dragon);
                if let Some(mut new_new_new_new_dragon) = new_new_new_dragon.try_reproduce() {
                    new_new_new_new_dragon.eat(50);
                    dragons_only(&new_new_new_new_dragon);
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

fn dragons_only(dragon: &impl DragonTrait) {
    dragon.fire();
}
