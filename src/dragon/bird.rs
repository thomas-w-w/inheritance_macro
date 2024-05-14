pub(crate) mod animal;

use crate::dragon::bird::animal::obj::*;
use animal::{AnimalArchetype, AnimalTrait};

#[derive(Clone)]
pub(crate) struct BirdArchetype {
    pub(crate) animal: AnimalArchetype,
    pub(crate) eggs: u32,
}

impl BirdArchetype {
    pub(crate) fn peep(&self) {
        println!("BirdArchetype::peep");
    }

    pub(crate) fn try_reproduce(&mut self) -> Option<BirdArchetype> {
        if self.eggs > 0 {
            self.animal
                .calories
                .checked_sub(50)
                .map(|remaining_calories| {
                    self.animal.calories = remaining_calories;
                    self.eggs -= 1;
                    BirdArchetype {
                        animal: self.animal.clone(),
                        eggs: self.eggs,
                    }
                })
        } else {
            None
        }
    }
}

pub(crate) trait BirdTrait: AnimalTrait {
    // type Offspring;
    fn peep(&self);
    // fn try_reproduce(&mut self) -> Option<Self::Offspring>;
}

struct Bird {
    bird: BirdArchetype,
}

impl Bird {
    pub fn new(bird: BirdArchetype) -> Self {
        Self { bird }
    }
}

impl ObjTrait for Bird {}

impl AnimalTrait for Bird {
    type Offspring = Bird;
    fn eat(&mut self, calories: u32) {
        self.bird.animal.eat(calories)
    }
    fn try_reproduce(&mut self) -> Option<Self::Offspring> {
        self.bird
            .try_reproduce()
            .map(|bird| Self::Offspring { bird })
    }
}

impl BirdTrait for Bird {
    fn peep(&self) {
        self.bird.peep()
    }
}

pub fn bird_main() {
    let mut bird = Bird::new(BirdArchetype {
        animal: AnimalArchetype {
            obj: ObjArchetype {
                obj_id: "1".to_string(),
                obj_type: ObjType::Bird,
            },
            calories: 10,
        },
        eggs: 3,
    });
    bird.peep();
    bird.eat(50);
    if let Some(mut new_bird) = bird.try_reproduce() {
        birds_only(&bird);
        birds_only(&new_bird);
        new_bird.eat(50);
    }
}

fn birds_only(bird: &impl BirdTrait) {
    bird.peep();
}
