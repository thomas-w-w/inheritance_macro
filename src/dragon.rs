pub(crate) mod bird;
pub(crate) mod lizard;

use crate::dragon::bird::animal::obj::*;
use crate::dragon::bird::animal::*;

use bird::BirdArchetype;
use bird::BirdTrait;

use lizard::LizardArchetype;
use lizard::LizardTrait;

struct DragonArchetype {
    etanol_liters: u32,
}

impl DragonArchetype {
    fn fire(&self) {
        println!("DragonArchetype::fire");
    }
    fn try_reproduce(
        &mut self,
        bird: &mut BirdArchetype,
        lizard: &mut LizardArchetype,
        animal: &mut AnimalArchetype,
    ) -> Option<DragonArchetype> {
        if bird.eggs > 0 {
            animal.calories.checked_sub(50).map(|remaining_calories| {
                animal.calories = remaining_calories;
                bird.eggs -= 1;
                DragonArchetype {
                    etanol_liters: self.etanol_liters.clone(),
                }
            })
        } else {
            None
        }
    }
}

trait DragonTrait: BirdTrait + LizardTrait {
    fn fire(&self);
}

struct Dragon {
    dragon: DragonArchetype,
    bird: BirdArchetype,
    lizard: LizardArchetype,
    animal: AnimalArchetype,
    obj: ObjArchetype,
}

impl Dragon {
    pub fn new(
        dragon: DragonArchetype,
        bird: BirdArchetype,
        lizard: LizardArchetype,
        animal: AnimalArchetype,
        obj: ObjArchetype,
    ) -> Self {
        Self {
            dragon,
            bird,
            lizard,
            animal,
            obj,
        }
    }
}

impl ObjTrait for Dragon {}

impl AnimalTrait for Dragon {
    type Offspring = Dragon;
    fn eat(&mut self, calories: u32) {
        self.animal.eat(calories)
    }

    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        self.dragon
            .try_reproduce(&mut self.bird, &mut self.lizard, &mut self.animal)
            .map(|dragon: DragonArchetype| Self::Offspring {
                dragon,
                bird: self.bird.clone(),
                lizard: self.lizard.clone(),
                animal: self.animal.clone(),
                obj: self.obj.clone(),
            })
    }
}

impl BirdTrait for Dragon {
    fn peep(&self) {
        self.bird.peep()
    }
}

impl LizardTrait for Dragon {
    fn crawl(&self) {
        self.lizard.crawl();
    }
}

impl DragonTrait for Dragon {
    fn fire(&self) {
        self.dragon.fire();
    }
}

pub fn dragon_main() {
    let mut dragon = Dragon::new(
        DragonArchetype {
            etanol_liters: 1000,
        },
        BirdArchetype { eggs: 3 },
        LizardArchetype { eggs: 3 },
        AnimalArchetype { calories: 10 },
        ObjArchetype {
            obj_id: "dragon#1".to_string(),
            obj_type: ObjType::Dragon,
        },
    );
    dragon.fire();
    dragon.eat(50);
    dragon.peep();
    dragon.crawl();
    if let Some(mut new_dragon) = dragon.try_reproduce() {
        dragons_only(&dragon);
        dragons_only(&new_dragon);
        new_dragon.eat(50);
    }
}

fn dragons_only(dragon: &impl DragonTrait) {
    dragon.fire();
}
