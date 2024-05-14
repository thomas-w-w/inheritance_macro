pub(crate) mod bird;
pub(crate) mod lizard;

use crate::dragon::bird::animal::obj::*;
use crate::dragon::bird::animal::*;

use bird::BirdArchetype;
use bird::BirdTrait;

use lizard::LizardArchetype;
use lizard::LizardTrait;

struct DragonArchetype {
    bird: BirdArchetype,
    lizard: LizardArchetype,
}

impl DragonArchetype {
    fn fire(&self) {
        println!("DragonArchetype::fire");
    }
    fn try_reproduce(&mut self) -> Option<DragonArchetype> {
        if self.bird.eggs > 0 {
            self.bird
                .animal
                .calories
                .checked_sub(50)
                .map(|remaining_calories| {
                    self.bird.animal.calories = remaining_calories;
                    self.bird.eggs -= 1;
                    DragonArchetype {
                        bird: self.bird.clone(),
                        lizard: self.lizard.clone(),
                    }
                })
        } else {
            None
        }
    }
}

trait DragonTrait: BirdTrait + LizardTrait {
    // fn try_reproduce(&mut self) -> Option<Dragon>;
    fn fire(&self);
}

struct Dragon {
    dragon: DragonArchetype,
}

impl Dragon {
    pub fn new(dragon: DragonArchetype) -> Self {
        Self { dragon }
    }
}

impl ObjTrait for Dragon {}

impl AnimalTrait for Dragon {
    type Offspring = Dragon;
    fn eat(&mut self, calories: u32) {
        self.dragon.bird.animal.eat(calories)
    }
    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        self.dragon
            .try_reproduce()
            .map(|dragon: DragonArchetype| Self::Offspring { dragon })
    }
}

impl BirdTrait for Dragon {
    fn peep(&self) {
        self.dragon.bird.peep()
    }
}

impl LizardTrait for Dragon {
    fn crawl(&self) {
        self.dragon.lizard.crawl();
    }
}

impl DragonTrait for Dragon {
    fn fire(&self) {
        self.dragon.fire();
    }
}

pub fn dragon_main() {
    let mut dragon = Dragon::new(DragonArchetype {
        bird: BirdArchetype {
            animal: AnimalArchetype {
                obj: ObjArchetype {
                    obj_id: "1".to_string(),
                    obj_type: ObjType::Dragon,
                },
                calories: 10,
            },
            eggs: 3,
        },
        lizard: LizardArchetype {
            animal: AnimalArchetype {
                obj: ObjArchetype {
                    obj_id: "1".to_string(),
                    obj_type: ObjType::Dragon,
                },
                calories: 10,
            },
            eggs: 3,
        },
    });
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
